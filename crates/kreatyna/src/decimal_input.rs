//! Decimal text as you type it: "1.5" or "1,5", at most 6 decimal places.
//! The catalog's quantities and a recipe ingredient's amount (REC-08) use
//! these rules. Each caller adds its own sign rule ("more than 0" or "not
//! negative").
//!
//! `rust_decimal` makes the value, but alone it refuses "1,5", accepts
//! "1_000", ".5", "1." and "+2", and gives one error with English text. So
//! a check of the characters and the decimal places runs first
//! (REC-08/Q4).
use rust_decimal::Decimal;

/// Why a text is not a decimal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DecimalInputError {
    /// "lots", "1.2.3", "1." or "".
    NotANumber,
    /// "0.1234567".
    TooManyDecimalPlaces,
    /// A number too large for `Decimal`.
    OutOfRange,
}

/// The decimal that a trimmed `input` names. A `-` in front is allowed, so
/// that the caller can say "must be positive" instead of "not a number".
pub(crate) fn parse_decimal(input: &str) -> Result<Decimal, DecimalInputError> {
    let unsigned = input.strip_prefix('-').unwrap_or(input);
    let mut parts = unsigned.split(['.', ',']);
    let integer = parts.next().unwrap_or_default();
    let fractional = parts.next();
    let digits = |text: &str| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(integer) || fractional.is_some_and(|part| !digits(part)) || parts.next().is_some() {
        return Err(DecimalInputError::NotANumber);
    }
    if fractional.is_some_and(|part| part.len() > 6) {
        return Err(DecimalInputError::TooManyDecimalPlaces);
    }
    Decimal::from_str_exact(&input.replace(',', ".")).map_err(|_| DecimalInputError::OutOfRange)
}
