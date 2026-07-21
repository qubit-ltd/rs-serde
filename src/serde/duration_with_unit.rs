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
//! Deserialization accepts strict strings with `ns`, `us`, `µs`, `μs`, `ms`,
//! `s`, `min`, `h`, or `d` suffixes.

use std::time::Duration;

use qubit_datatype::{
    format_duration_exact,
    parse_duration_text,
    DurationParseError,
    DurationTextOptions,
    DurationUnitParseMode,
    SuffixlessDurationPolicy,
};
use serde::de::Error as DeserializeError;
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

/// Strict Duration text profile.
const DURATION_TEXT_OPTIONS: DurationTextOptions = DurationTextOptions::new(
    SuffixlessDurationPolicy::Reject,
    DurationUnitParseMode::Strict,
);

/// Serializes a [`Duration`] as an exact string such as `"500µs"`.
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

/// Deserializes a [`Duration`] from an exact strict unit-suffixed string.
///
/// # Parameters
/// - `deserializer`: Serde deserializer providing a string value.
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
    let text = String::deserialize(deserializer)?;
    parse(&text).map_err(DeserializeError::custom)
}

/// Formats a [`Duration`] with the largest exact supported unit.
///
/// # Parameters
/// - `duration`: Duration to format.
///
/// # Returns
/// A preferred unit-suffixed string. Zero is formatted as `0ms`.
#[inline]
pub fn format(duration: &Duration) -> String {
    format_duration_exact(*duration)
}

/// Parses a [`Duration`] from a string with a supported unit.
///
/// Supported strict suffixes are `ns`, `us`, `µs`, `μs`, `ms`, `s`, `min`,
/// `h`, and `d`. Bare integers and the Lenient-only `m` alias are rejected.
///
/// # Parameters
/// - `text`: Duration text to parse.
///
/// # Returns
/// The parsed [`Duration`].
///
/// # Errors
/// Returns [`DurationParseError::InvalidSyntax`] for malformed text,
/// [`DurationParseError::NonCanonicalUnit`] for a supported alias,
/// [`DurationParseError::UnsupportedUnit`] for an unknown unit, and
/// [`DurationParseError::OutOfRange`] when the value cannot fit in a
/// [`Duration`].
#[inline(always)]
pub fn parse(text: &str) -> Result<Duration, DurationParseError> {
    parse_duration_text(text, &DURATION_TEXT_OPTIONS)
}
