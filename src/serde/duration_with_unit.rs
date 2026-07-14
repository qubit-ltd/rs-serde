// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Exact Serde adapter for [`std::time::Duration`] as a unit-suffixed string.
//!
//! Serialization selects the largest supported unit that represents the value
//! without losing precision. Zero is represented as `0ms`.
//! Deserialization accepts strings with `ns`, `us`, `ms`, `s`, `m`, `h`, or
//! `d` suffixes, and also accepts a bare integer as milliseconds
//! for lenient configuration input.

use std::fmt;
use std::time::Duration;

use serde::de::{
    Error as DeserializeError,
    Unexpected,
    Visitor,
};
use serde::{
    Deserializer,
    Serializer,
};

pub use super::parse_duration_error::ParseDurationError;

/// Units larger than nanoseconds, in descending order of magnitude.
const EXACT_UNITS: [(u128, &str); 6] = [
    (86_400_000_000_000, "d"),
    (3_600_000_000_000, "h"),
    (60_000_000_000, "m"),
    (1_000_000_000, "s"),
    (1_000_000, "ms"),
    (1_000, "us"),
];

/// Supported units in parsed duration text.
#[derive(Clone, Copy)]
enum DurationUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl DurationUnit {
    /// Converts a unit count to a duration without overflowing its seconds.
    ///
    /// # Parameters
    ///
    /// - `value`: Non-negative count expressed in this unit.
    ///
    /// # Returns
    ///
    /// The represented duration, or [`None`] when it is out of range.
    fn duration_from_u128(self, value: u128) -> Option<Duration> {
        match self {
            Self::Nanoseconds => {
                duration_from_subseconds(value, 1_000_000_000, 1)
            }
            Self::Microseconds => {
                duration_from_subseconds(value, 1_000_000, 1_000)
            }
            Self::Milliseconds => {
                duration_from_subseconds(value, 1_000, 1_000_000)
            }
            Self::Seconds => duration_from_seconds(value, 1),
            Self::Minutes => duration_from_seconds(value, 60),
            Self::Hours => duration_from_seconds(value, 3_600),
            Self::Days => duration_from_seconds(value, 86_400),
        }
    }
}

/// Converts a subsecond unit count to a duration.
fn duration_from_subseconds(
    value: u128,
    units_per_second: u128,
    nanos_per_unit: u32,
) -> Option<Duration> {
    let seconds = u64::try_from(value / units_per_second).ok()?;
    let subsecond_units = u32::try_from(value % units_per_second).ok()?;
    let nanoseconds = subsecond_units.checked_mul(nanos_per_unit)?;
    Some(Duration::new(seconds, nanoseconds))
}

/// Converts a whole-unit count to a duration in seconds.
fn duration_from_seconds(
    value: u128,
    seconds_per_unit: u128,
) -> Option<Duration> {
    let seconds = value.checked_mul(seconds_per_unit)?;
    Some(Duration::from_secs(u64::try_from(seconds).ok()?))
}

/// Visitor for the shared unit-suffixed duration input protocol.
struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
    type Value = Duration;

    #[inline(always)]
    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "a duration string with an optional unit or a non-negative \
             millisecond integer",
        )
    }

    #[inline(always)]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        parse(value).map_err(E::custom)
    }

    #[inline(always)]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        self.visit_str(&value)
    }

    #[inline(always)]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        duration_from_millis(u128::from(value))
    }

    #[inline(always)]
    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        duration_from_millis(value)
    }

    #[inline(always)]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        match u64::try_from(value) {
            Ok(value) => self.visit_u64(value),
            Err(_) => Err(E::invalid_value(Unexpected::Signed(value), &self)),
        }
    }

    #[inline(always)]
    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: DeserializeError,
    {
        match u128::try_from(value) {
            Ok(value) => self.visit_u128(value),
            Err(_) => {
                Err(E::custom("duration integer must be a non-negative value"))
            }
        }
    }
}

/// Serializes a [`Duration`] as an exact string such as `"500us"`.
///
/// # Parameters
/// - `duration`: Duration to serialize.
/// - `serializer`: Serde serializer receiving the formatted string.
///
/// # Returns
/// The serializer result.
///
/// # Errors
/// Returns the serializer error if writing the string value fails.
#[inline]
pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let text = format(duration);
    serializer.serialize_str(&text)
}

/// Deserializes a [`Duration`] from an exact unit-suffixed string, or a bare
/// millisecond integer.
///
/// # Parameters
/// - `deserializer`: Serde deserializer providing a string or integer value.
///
/// # Returns
/// The parsed [`Duration`].
///
/// # Errors
/// Returns the deserializer error when the input has an unsupported unit,
/// invalid number, fractional value, or overflows [`Duration`].
#[inline]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_any(DurationVisitor)
    } else {
        deserializer.deserialize_str(DurationVisitor)
    }
}

/// Formats a [`Duration`] with the largest exact supported unit.
///
/// # Parameters
/// - `duration`: Duration to format.
///
/// # Returns
/// A canonical unit-suffixed string. Zero is formatted as `0ms`.
#[inline]
pub fn format(duration: &Duration) -> String {
    let total_nanos = duration.as_nanos();
    if total_nanos == 0 {
        return "0ms".to_string();
    }
    for (nanos_per_unit, suffix) in EXACT_UNITS {
        if total_nanos.is_multiple_of(nanos_per_unit) {
            return format!("{}{suffix}", total_nanos / nanos_per_unit);
        }
    }
    format!("{total_nanos}ns")
}

/// Parses a [`Duration`] from a string with a supported unit.
///
/// Bare integers are treated as milliseconds. Supported suffixes are `ns`,
/// `us`, `ms`, `s`, `m`, `h`, and `d`.
///
/// # Parameters
/// - `text`: Duration text to parse.
///
/// # Returns
/// The parsed [`Duration`].
///
/// # Errors
/// Returns [`ParseDurationError::InvalidSyntax`] for non-canonical text,
/// [`ParseDurationError::UnsupportedUnit`] for an unknown ASCII unit, and
/// [`ParseDurationError::OutOfRange`] when the value cannot fit in a
/// [`Duration`].
pub fn parse(text: &str) -> Result<Duration, ParseDurationError> {
    let split_at = text
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(text.len());
    let (digits, suffix) = text.split_at(split_at);
    if digits.is_empty() {
        return Err(ParseDurationError::InvalidSyntax);
    }
    let unit = parse_unit(suffix)?;
    let value = digits
        .parse::<u128>()
        .map_err(|_| ParseDurationError::OutOfRange)?;
    unit.duration_from_u128(value)
        .ok_or(ParseDurationError::OutOfRange)
}

/// Parses a canonical duration unit suffix, defaulting a missing suffix to
/// milliseconds.
///
/// # Parameters
///
/// - `suffix`: The suffix portion following the numeric duration value.
///
/// # Returns
///
/// The corresponding duration unit.
///
/// # Errors
///
/// Returns [`ParseDurationError::InvalidSyntax`] for non-ASCII syntax and
/// [`ParseDurationError::UnsupportedUnit`] for an unknown ASCII suffix.
fn parse_unit(suffix: &str) -> Result<DurationUnit, ParseDurationError> {
    let unit = match suffix {
        "" | "ms" => DurationUnit::Milliseconds,
        "ns" => DurationUnit::Nanoseconds,
        "us" => DurationUnit::Microseconds,
        "s" => DurationUnit::Seconds,
        "m" => DurationUnit::Minutes,
        "h" => DurationUnit::Hours,
        "d" => DurationUnit::Days,
        _ if suffix.bytes().all(|byte| byte.is_ascii_alphabetic()) => {
            return Err(ParseDurationError::UnsupportedUnit {
                unit: suffix.to_string(),
            });
        }
        _ => return Err(ParseDurationError::InvalidSyntax),
    };
    Ok(unit)
}

/// Converts a non-negative millisecond count from a Serde integer token.
///
/// # Parameters
///
/// - `millis`: Millisecond count supplied by a human-readable deserializer.
///
/// # Returns
///
/// The represented duration.
///
/// # Errors
///
/// Returns the deserializer's custom error when the count exceeds
/// [`Duration`]'s range.
fn duration_from_millis<E>(millis: u128) -> Result<Duration, E>
where
    E: DeserializeError,
{
    DurationUnit::Milliseconds
        .duration_from_u128(millis)
        .ok_or_else(|| E::custom(ParseDurationError::OutOfRange))
}
