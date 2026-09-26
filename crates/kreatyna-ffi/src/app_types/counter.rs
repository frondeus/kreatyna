use kreatyna as core;

use super::AppTypeConversionError;
use super::support::count;

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum CounterEvent {
    Increase,
    Decrease,
    Reset,
    StartBackground,
    StopBackground,
}

impl From<CounterEvent> for core::CounterEvent {
    fn from(value: CounterEvent) -> Self {
        match value {
            CounterEvent::Increase => Self::Increase,
            CounterEvent::Decrease => Self::Decrease,
            CounterEvent::Reset => Self::Reset,
            CounterEvent::StartBackground => Self::StartBackground,
            CounterEvent::StopBackground => Self::StopBackground,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum CounterStatus {
    Loading,
    Ready,
}

impl From<core::CounterStatus> for CounterStatus {
    fn from(value: core::CounterStatus) -> Self {
        match value {
            core::CounterStatus::Loading => Self::Loading,
            core::CounterStatus::Ready => Self::Ready,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Record)]
pub struct CounterView {
    pub count: u64,
    pub status: CounterStatus,
}

impl TryFrom<core::CounterView> for CounterView {
    type Error = AppTypeConversionError;

    fn try_from(value: core::CounterView) -> Result<Self, Self::Error> {
        Ok(Self {
            count: count(value.count)?,
            status: value.status.into(),
        })
    }
}
