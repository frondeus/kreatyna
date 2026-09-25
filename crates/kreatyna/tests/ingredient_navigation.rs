use kreatyna::ingredients::{
    IngredientEditOrigin as Origin, IngredientId, IngredientNavigation as Navigation,
    IngredientNavigationAction as Action, IngredientNavigationError as Error,
};
use test_case::test_case;
use uuid::Uuid;

#[test]
fn default_navigation_is_the_list() {
    assert_eq!(Navigation::default(), Navigation::List);
}

#[test_case(Navigation::List, Action::OpenCreate, Navigation::Create; "open creation")]
#[test_case(Navigation::List, Action::OpenDetail { id: id(1) }, detail(1); "open detail")]
#[test_case(detail(1), Action::OpenDetail { id: id(2) }, detail(2); "replace detail")]
#[test_case(detail(1), Action::OpenDetail { id: id(1) }, detail(1); "reopen same detail")]
#[test_case(Navigation::List, Action::OpenEdit { id: id(1) }, edit(Origin::List); "edit from list")]
#[test_case(detail(1), Action::OpenEdit { id: id(1) }, edit(Origin::Detail); "edit from detail")]
#[test_case(detail(1), Action::CloseDetail, Navigation::List; "close detail")]
#[test_case(Navigation::Create, Action::FormDiscarded, Navigation::List; "discard creation")]
#[test_case(Navigation::Create, Action::FormSaved { id: id(2) }, Navigation::List; "save creation")]
#[test_case(edit(Origin::List), Action::FormDiscarded, Navigation::List; "discard direct edit")]
#[test_case(edit(Origin::Detail), Action::FormDiscarded, detail(1); "discard detail edit")]
#[test_case(edit(Origin::List), Action::FormSaved { id: id(1) }, Navigation::List; "save direct edit")]
#[test_case(edit(Origin::Detail), Action::FormSaved { id: id(1) }, detail(1); "save detail edit")]
#[test_case(detail(1), Action::IngredientDeleted { id: id(1) }, Navigation::List; "delete displayed ingredient")]
fn allowed_transitions_return_the_expected_history(
    source: Navigation,
    action: Action,
    expected: Navigation,
) {
    assert_eq!(source.transition(action), Ok(expected));
}

#[test_case(Navigation::List, Action::CloseDetail; "list cannot close detail")]
#[test_case(Navigation::List, Action::FormDiscarded; "list cannot discard form")]
#[test_case(Navigation::List, Action::FormSaved { id: id(1) }; "list cannot complete save")]
#[test_case(Navigation::List, Action::IngredientDeleted { id: id(1) }; "list cannot complete deletion")]
#[test_case(detail(1), Action::OpenCreate; "detail cannot open creation")]
#[test_case(detail(1), Action::FormDiscarded; "detail cannot discard form")]
#[test_case(detail(1), Action::FormSaved { id: id(1) }; "detail cannot complete save")]
#[test_case(Navigation::Create, Action::OpenCreate; "creation cannot reopen creation")]
#[test_case(Navigation::Create, Action::OpenDetail { id: id(1) }; "creation cannot open detail")]
#[test_case(Navigation::Create, Action::OpenEdit { id: id(1) }; "creation cannot open edit")]
#[test_case(Navigation::Create, Action::CloseDetail; "creation cannot close detail")]
#[test_case(Navigation::Create, Action::IngredientDeleted { id: id(1) }; "creation cannot complete deletion")]
#[test_case(edit(Origin::List), Action::OpenCreate; "direct edit cannot open creation")]
#[test_case(edit(Origin::List), Action::OpenDetail { id: id(2) }; "direct edit cannot open detail")]
#[test_case(edit(Origin::List), Action::OpenEdit { id: id(1) }; "direct edit cannot reopen edit")]
#[test_case(edit(Origin::List), Action::CloseDetail; "direct edit cannot close detail")]
#[test_case(edit(Origin::List), Action::IngredientDeleted { id: id(1) }; "direct edit cannot complete deletion")]
#[test_case(edit(Origin::Detail), Action::OpenCreate; "detail edit cannot open creation")]
#[test_case(edit(Origin::Detail), Action::OpenDetail { id: id(1) }; "detail edit cannot open detail")]
#[test_case(edit(Origin::Detail), Action::OpenEdit { id: id(2) }; "detail edit cannot switch edit")]
#[test_case(edit(Origin::Detail), Action::CloseDetail; "detail edit cannot close detail")]
#[test_case(edit(Origin::Detail), Action::IngredientDeleted { id: id(1) }; "detail edit cannot complete deletion")]
fn unsupported_actions_leave_the_source_unchanged(source: Navigation, action: Action) {
    let before = source;
    assert_eq!(source.transition(action), Err(Error::InvalidTransition));
    assert_eq!(source, before);
}

#[test_case(detail(1), Action::OpenEdit { id: id(2) }; "edit another ingredient under detail")]
#[test_case(edit(Origin::List), Action::FormSaved { id: id(2) }; "direct edit save has wrong id")]
#[test_case(edit(Origin::Detail), Action::FormSaved { id: id(2) }; "detail edit save has wrong id")]
#[test_case(detail(1), Action::IngredientDeleted { id: id(2) }; "delete another ingredient")]
fn mismatched_ingredients_leave_the_source_unchanged(source: Navigation, action: Action) {
    let before = source;
    assert_eq!(source.transition(action), Err(Error::IngredientMismatch));
    assert_eq!(source, before);
}

#[test_case(Navigation::List, None; "root has no back destination")]
#[test_case(detail(1), Some(Navigation::List); "detail returns to list")]
#[test_case(Navigation::Create, Some(Navigation::List); "creation returns to list")]
#[test_case(edit(Origin::List), Some(Navigation::List); "direct edit returns to list")]
#[test_case(edit(Origin::Detail), Some(detail(1)); "detail edit returns to its ingredient")]
fn back_is_a_destination_query(source: Navigation, expected: Option<Navigation>) {
    let before = source;
    assert_eq!(source.back_destination(), expected);
    assert_eq!(source, before);
}

fn id(value: u128) -> IngredientId {
    Uuid::from_u128(value).into()
}

fn detail(value: u128) -> Navigation {
    Navigation::Detail { id: id(value) }
}

fn edit(origin: Origin) -> Navigation {
    Navigation::Edit { id: id(1), origin }
}
