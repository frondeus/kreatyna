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
