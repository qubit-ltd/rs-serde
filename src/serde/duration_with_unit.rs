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

use qubit_datatype::{
    DurationTextOptions,
    DurationUnit,
    DurationUnitSuffixSet,
    SuffixlessDurationPolicy,
    format_duration_exact,
    parse_duration_text,
};
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

/// ASCII Duration text profile with suffixless milliseconds.
const DURATION_TEXT_OPTIONS: DurationTextOptions = DurationTextOptions::new(
    SuffixlessDurationPolicy::Assume(DurationUnit::Milliseconds),
    DurationUnitSuffixSet::Ascii,
);

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
    format_duration_exact(*duration)
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
#[inline(always)]
pub fn parse(text: &str) -> Result<Duration, ParseDurationError> {
    parse_duration_text(text, &DURATION_TEXT_OPTIONS)
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
#[inline(always)]
fn duration_from_millis<E>(millis: u128) -> Result<Duration, E>
where
    E: DeserializeError,
{
    DurationUnit::Milliseconds
        .duration_from_u128(millis)
        .map_err(|_| E::custom(ParseDurationError::OutOfRange))
}
