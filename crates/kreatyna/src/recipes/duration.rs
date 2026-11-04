//! A step's duration (REC-17): what the inspector's Duration field sends,
//! the range check, and the text that the node and the read view show.
use super::{StepDurationS, StepDurationSError};

const MINUTE: u32 = 60;
const HOUR: u32 = 60 * MINUTE;
const DAY: u32 = 24 * HOUR;

/// What the Duration field sends (REC-17/Q1, variant E).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationInput {
    /// An empty minutes field.
    Cleared,
    /// A chip, or the minutes field.
    Minutes(u32),
    /// The unit fields under More….
    Parts(DurationParts),
}

/// A duration split into units. `StepDurationS::parts` fills each unit in
/// its range (0–23 h, 0–59 min, 0–59 s). The unit fields send it back as
/// typed, and a field may then hold more, such as 75 min: it adds up all
/// the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DurationParts {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

/// How the Duration field shows a set duration (REC-17/Q1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationEntry {
    /// Whole minutes, less than a day. The minutes field shows them, the
    /// chip with the same minutes is on, and More… opens only on a click.
    Minutes { minutes: u32 },
    /// Whole minutes, a day or more. The minutes field shows them, and the
    /// unit fields open by themselves.
    MinutesWithDays { minutes: u32 },
    /// The duration has seconds. The minutes field shows "…", and the unit
    /// fields open by themselves.
    Seconds,
}

/// One chip of the Duration field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationChip {
    /// What a click sends, as `DurationInput::Minutes`.
    pub minutes: u32,
    /// "10 min", "1 h".
    pub text: String,
}

/// The chips, shortest first (REC-17/Q1).
const CHIP_MINUTES: [u32; 9] = [1, 2, 5, 10, 15, 20, 30, 45, 60];

/// The Duration field's chips, shortest first. Each shows its length as the
/// node shows it: 60 is "1 h".
pub(crate) fn duration_chips() -> Vec<DurationChip> {
    CHIP_MINUTES
        .into_iter()
        .filter_map(|minutes| {
            let duration = StepDurationS::try_new(minutes * MINUTE).ok()?;
            Some(DurationChip {
                minutes,
                text: duration.text(),
            })
        })
        .collect()
}

/// Why a duration cannot be saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationError {
    /// More than 365 days (REC-17/Q4).
    TooLong,
}

impl DurationInput {
    /// The duration this input sets. `Ok(None)` for `Cleared` and for 0,
    /// which mean "no duration". Adds in `u64`, so even the largest fields
    /// give `TooLong`, never an overflow.
    pub fn duration(self) -> Result<Option<StepDurationS>, DurationError> {
        let total = match self {
            Self::Cleared => 0,
            Self::Minutes(minutes) => u64::from(minutes) * u64::from(MINUTE),
            Self::Parts(parts) => parts.total_seconds(),
        };
        let seconds = u32::try_from(total).map_err(|_| DurationError::TooLong)?;
        match StepDurationS::try_new(seconds) {
            Ok(duration) => Ok(Some(duration)),
            Err(StepDurationSError::GreaterViolated) => Ok(None),
            Err(StepDurationSError::LessOrEqualViolated) => Err(DurationError::TooLong),
        }
    }
}

impl DurationInput {
    /// The input as unit fields, so a duration that is too long still shows
    /// what you set. A cleared field is all zeros; minutes go to the
    /// minutes field.
    pub(crate) fn parts(self) -> DurationParts {
        match self {
            Self::Cleared => DurationParts::default(),
            Self::Minutes(minutes) => DurationParts {
                minutes,
                ..DurationParts::default()
            },
            Self::Parts(parts) => parts,
        }
    }
}

impl DurationParts {
    fn total_seconds(self) -> u64 {
        u64::from(self.days) * u64::from(DAY)
            + u64::from(self.hours) * u64::from(HOUR)
            + u64::from(self.minutes) * u64::from(MINUTE)
            + u64::from(self.seconds)
    }
}

impl StepDurationS {
    /// The duration in days, hours, minutes, and seconds, each in its range.
    pub fn parts(self) -> DurationParts {
        let seconds = self.into_inner();
        DurationParts {
            days: seconds / DAY,
            hours: seconds % DAY / HOUR,
            minutes: seconds % HOUR / MINUTE,
            seconds: seconds % MINUTE,
        }
    }

    /// How the Duration field shows this duration.
    pub fn entry(self) -> DurationEntry {
        let seconds = self.into_inner();
        let minutes = seconds / MINUTE;
        if !seconds.is_multiple_of(MINUTE) {
            DurationEntry::Seconds
        } else if seconds < DAY {
            DurationEntry::Minutes { minutes }
        } else {
            DurationEntry::MinutesWithDays { minutes }
        }
    }

    /// The largest units first, without the units that are 0: "45 s",
    /// "1 min 30 s", "1 h 30 min", "2 d".
    pub fn text(self) -> String {
        let parts = self.parts();
        [
            (parts.days, "d"),
            (parts.hours, "h"),
            (parts.minutes, "min"),
            (parts.seconds, "s"),
        ]
        .into_iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, unit)| format!("{count} {unit}"))
        .collect::<Vec<_>>()
        .join(" ")
    }
}
