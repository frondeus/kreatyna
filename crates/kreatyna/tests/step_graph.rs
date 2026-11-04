//! The step graph (REC-18): which arrows `StepEdges` accepts and which it
//! refuses.
use std::collections::BTreeMap;

use kreatyna::recipes::{ConnectRefusal, StepEdge, StepEdges, StepId};
use test_case::test_case;
use uuid::Uuid;

// Green path

#[test_case(&[]; "no edges")]
#[test_case(&[(1, 2), (2, 3), (3, 4)]; "a chain")]
#[test_case(&[(1, 2), (1, 3), (2, 4), (3, 4)]; "a diamond")]
#[test_case(&[(1, 2), (3, 4)]; "loose ends")]
#[test_case(&[(1, 2), (2, 3), (1, 3)]; "a shortcut past a step")]
fn connect_accepts_edges_without_a_loop(pairs: &[(u128, u128)]) -> anyhow::Result<()> {
    let mut edges = StepEdges::default();
    for &pair in pairs {
        edges.connect(edge(pair))?;
    }
    assert_eq!(collect(&edges), sorted(pairs.iter().copied().map(edge)));
    Ok(())
}

#[test]
fn the_edges_are_a_set_in_one_order() -> anyhow::Result<()> {
    let forward = StepEdges::from_saved([edge((1, 2)), edge((3, 4))])?;
    let backward = StepEdges::from_saved([edge((3, 4)), edge((1, 2))])?;
    assert_eq!(forward, backward);
    assert_eq!(collect(&forward), collect(&backward));
    Ok(())
}

// Validation

#[test_case(&[], (1, 1), ConnectRefusal::SameStep; "a step to itself")]
#[test_case(&[(1, 2)], (1, 2), ConnectRefusal::Duplicate; "the same edge twice")]
#[test_case(&[(1, 2)], (2, 1), ConnectRefusal::Loop; "a loop of 2 steps")]
#[test_case(&[(1, 2), (2, 3)], (3, 1), ConnectRefusal::Loop; "a loop of 3 steps")]
#[test_case(&[(1, 2), (2, 3), (3, 4)], (4, 1), ConnectRefusal::Loop; "a loop of 4 steps")]
#[test_case(&[(1, 2), (1, 3), (2, 4), (3, 4)], (4, 1), ConnectRefusal::Loop; "a loop through a diamond")]
fn connect_refuses_and_changes_nothing(
    pairs: &[(u128, u128)],
    refused: (u128, u128),
    refusal: ConnectRefusal,
) -> anyhow::Result<()> {
    let mut edges = StepEdges::from_saved(pairs.iter().copied().map(edge))?;
    let before = edges.clone();
    assert_eq!(edges.connect(edge(refused)), Err(refusal));
    assert_eq!(edges, before);
    Ok(())
}

#[test_case(&[(1, 1)], ConnectRefusal::SameStep; "a step to itself")]
#[test_case(&[(1, 2), (1, 2)], ConnectRefusal::Duplicate; "the same edge twice")]
#[test_case(&[(1, 2), (2, 1)], ConnectRefusal::Loop; "a loop of 2 steps")]
#[test_case(&[(1, 2), (2, 3), (3, 1)], ConnectRefusal::Loop; "a loop of 3 steps")]
#[test_case(&[(1, 2), (2, 3), (3, 4), (4, 1)], ConnectRefusal::Loop; "a loop of 4 steps")]
fn from_saved_refuses_what_connect_refuses(pairs: &[(u128, u128)], refusal: ConnectRefusal) {
    assert_eq!(
        StepEdges::from_saved(pairs.iter().copied().map(edge)),
        Err(refusal)
    );
}

/// Every set of the 12 possible edges between 4 steps: 4,096 graphs.
/// `from_saved` adds the edges one by one with `connect`. It must accept a
/// set exactly when Kahn's algorithm can sort the set, that is, when the
/// set has no loop.
#[test]
fn from_saved_accepts_exactly_the_sets_without_a_loop() -> anyhow::Result<()> {
    let possible: Vec<StepEdge> = (1..=4)
        .flat_map(|from| (1..=4).map(move |to| (from, to)))
        .filter(|(from, to)| from != to)
        .map(edge)
        .collect();
    assert_eq!(possible.len(), 12);
    let mut accepted = 0;
    for mask in 0..1_u32 << possible.len() {
        let set: Vec<StepEdge> = possible
            .iter()
            .enumerate()
            .filter(|(bit, _)| mask & (1 << bit) != 0)
            .map(|(_, edge)| *edge)
            .collect();
        let saved = StepEdges::from_saved(set.iter().copied());
        let expected = if has_topological_order(&set) {
            Ok(set.len())
        } else {
            Err(ConnectRefusal::Loop)
        };
        assert_eq!(saved.map(|edges| edges.len()), expected, "{set:?}");
        accepted += usize::from(expected.is_ok());
    }
    // The number of acyclic directed graphs on 4 labelled nodes (OEIS A003024).
    assert_eq!(accepted, 543);
    Ok(())
}

// Test support

fn step(n: u128) -> StepId {
    StepId::from(Uuid::from_u128(n))
}

fn edge((from, to): (u128, u128)) -> StepEdge {
    StepEdge {
        from: step(from),
        to: step(to),
    }
}

fn collect(edges: &StepEdges) -> Vec<StepEdge> {
    edges.iter().copied().collect()
}

fn sorted(edges: impl IntoIterator<Item = StepEdge>) -> Vec<StepEdge> {
    let mut all: Vec<_> = edges.into_iter().collect();
    all.sort();
    all
}

/// Kahn's algorithm: remove steps with no arrow in, one at a time. The set
/// has a loop exactly when some steps are left.
fn has_topological_order(edges: &[StepEdge]) -> bool {
    let mut incoming: BTreeMap<StepId, usize> = BTreeMap::new();
    for edge in edges {
        incoming.entry(edge.from).or_default();
        *incoming.entry(edge.to).or_default() += 1;
    }
    let mut ready: Vec<StepId> = incoming
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(step, _)| *step)
        .collect();
    let mut sorted = 0;
    while let Some(step) = ready.pop() {
        sorted += 1;
        for edge in edges.iter().filter(|edge| edge.from == step) {
            if let Some(count) = incoming.get_mut(&edge.to) {
                *count -= 1;
                if *count == 0 {
                    ready.push(edge.to);
                }
            }
        }
    }
    sorted == incoming.len()
}
