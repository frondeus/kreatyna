use kreatyna::recipes::{
    DurationEntry, DurationError, DurationInput, DurationParts, StepDurationS,
};
use test_case::test_case;

// Green path

#[test_case(DurationInput::Minutes(10), 600; "a chip")]
#[test_case(DurationInput::Minutes(90), 5_400; "minutes past an hour")]
#[test_case(DurationInput::Parts(parts(0, 0, 1, 30)), 90; "minutes and seconds")]
#[test_case(DurationInput::Parts(parts(0, 0, 0, 45)), 45; "seconds only")]
#[test_case(DurationInput::Parts(parts(2, 0, 0, 0)), 172_800; "days")]
#[test_case(DurationInput::Parts(parts(0, 1, 75, 0)), 8_100; "a field past its unit adds up")]
#[test_case(DurationInput::Parts(parts(365, 0, 0, 0)), 31_536_000; "the longest, 365 days")]
fn an_input_sets_its_total_in_seconds(input: DurationInput, seconds: u32) -> anyhow::Result<()> {
    assert_eq!(input.duration(), Ok(Some(StepDurationS::try_new(seconds)?)));
    Ok(())
}

#[test_case(DurationInput::Cleared; "a cleared field")]
#[test_case(DurationInput::Minutes(0); "0 minutes")]
#[test_case(DurationInput::Parts(DurationParts::default()); "every unit field at 0")]
fn an_empty_input_is_no_duration(input: DurationInput) {
    assert_eq!(input.duration(), Ok(None));
}

#[test_case(45, parts(0, 0, 0, 45), "45 s"; "seconds")]
#[test_case(90, parts(0, 0, 1, 30), "1 min 30 s"; "minutes and seconds")]
#[test_case(360, parts(0, 0, 6, 0), "6 min"; "minutes")]
#[test_case(3_600, parts(0, 1, 0, 0), "1 h"; "one hour")]
#[test_case(5_400, parts(0, 1, 30, 0), "1 h 30 min"; "hours and minutes")]
#[test_case(28_800, parts(0, 8, 0, 0), "8 h"; "overnight")]
#[test_case(172_800, parts(2, 0, 0, 0), "2 d"; "days")]
#[test_case(90_061, parts(1, 1, 1, 1), "1 d 1 h 1 min 1 s"; "every unit")]
#[test_case(31_536_000, parts(365, 0, 0, 0), "365 d"; "the longest")]
fn a_duration_splits_into_units_and_shows_the_units_it_has(
    seconds: u32,
    expected: DurationParts,
    text: &str,
) -> anyhow::Result<()> {
    let duration = StepDurationS::try_new(seconds)?;
    assert_eq!(duration.parts(), expected);
    assert_eq!(duration.text(), text);
    Ok(())
}

#[test_case(60, DurationEntry::Minutes { minutes: 1 }; "one minute")]
#[test_case(5_400, DurationEntry::Minutes { minutes: 90 }; "past an hour")]
#[test_case(86_340, DurationEntry::Minutes { minutes: 1_439 }; "a minute short of a day")]
#[test_case(86_400, DurationEntry::MinutesWithDays { minutes: 1_440 }; "one day")]
#[test_case(172_800, DurationEntry::MinutesWithDays { minutes: 2_880 }; "days")]
#[test_case(90, DurationEntry::Seconds; "minutes and seconds")]
#[test_case(45, DurationEntry::Seconds; "seconds only")]
#[test_case(86_401, DurationEntry::Seconds; "days and seconds")]
fn the_duration_field_shows_minutes_unless_there_are_seconds_and_opens_the_units_for_days(
    seconds: u32,
    expected: DurationEntry,
) -> anyhow::Result<()> {
    assert_eq!(StepDurationS::try_new(seconds)?.entry(), expected);
    Ok(())
}

// Validation

#[test_case(DurationInput::Parts(parts(365, 0, 0, 1)); "one second past 365 days")]
#[test_case(DurationInput::Minutes(525_601); "minutes past 365 days")]
#[test_case(DurationInput::Parts(parts(u32::MAX, u32::MAX, u32::MAX, u32::MAX)); "fields too large for u32")]
fn an_input_past_365_days_is_too_long(input: DurationInput) {
    assert_eq!(input.duration(), Err(DurationError::TooLong));
}

// Test support

fn parts(days: u32, hours: u32, minutes: u32, seconds: u32) -> DurationParts {
    DurationParts {
        days,
        hours,
        minutes,
        seconds,
    }
}
