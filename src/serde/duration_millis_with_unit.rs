// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossy Serde adapter for [`std::time::Duration`] as millisecond text.
//!
//! Serialization rounds to the nearest whole millisecond using half-up
//! rounding and appends `ms`. Deserialization accepts only the matching
//! canonical `<integer>ms` form.

use std::time::Duration;

use qubit_datatype::{
    DurationParseError,
    DurationUnit,
};
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

use super::duration_millis::rounded_millis;

/// Deserializes only the canonical millisecond text representation.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    parse(&text).map_err(serde::de::Error::custom)
}

/// Parses a non-negative whole millisecond count with the canonical ms symbol.
pub fn parse(text: &str) -> Result<Duration, DurationParseError> {
    let Some(digits) = text.strip_suffix("ms") else {
        return Err(DurationParseError::InvalidSyntax);
    };
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(DurationParseError::InvalidSyntax);
    }
    let millis = digits
        .parse::<u128>()
        .map_err(|_| DurationParseError::OutOfRange)?;
    DurationUnit::Milliseconds
        .duration_from_u128(millis)
        .map_err(|_| DurationParseError::OutOfRange)
}

/// Serializes a [`Duration`] as rounded whole milliseconds with an `ms`
/// suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and serialize.
/// - `serializer`: Serde serializer receiving the formatted string.
///
/// # Returns
///
/// The serializer result.
///
/// # Errors
///
/// Returns the serializer error if writing the string value fails.
pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format(duration))
}

/// Formats a [`Duration`] as rounded whole milliseconds with an `ms` suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and format.
///
/// # Returns
///
/// A string in the form `<rounded-millis>ms`.
#[inline(always)]
pub fn format(duration: &Duration) -> String {
    let millis = rounded_millis(*duration);
    format!("{millis}ms")
}
