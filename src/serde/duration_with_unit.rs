/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Serde adapter for [`std::time::Duration`] as a string with a time unit.
//!
//! Serialization emits whole milliseconds with an `ms` suffix. Deserialization
//! accepts strings with `ns`, `us`, `µs`, `ms`, `s`, `m`, `h`, or `d` suffixes,
//! and also accepts a bare integer as milliseconds for lenient configuration
//! input.

use std::time::Duration;

use serde::de::Error;
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

use super::duration_millis::as_millis_u64;

/// Serializes a [`Duration`] as a string such as `"500ms"`.
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
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format(duration))
}

/// Deserializes a [`Duration`] from a string with a unit, or a bare millisecond
/// integer.
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
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(number) => {
            let millis = number
                .as_u64()
                .ok_or_else(|| D::Error::custom("duration integer must be a non-negative u64"))?;
            Ok(Duration::from_millis(millis))
        }
        serde_json::Value::String(text) => parse(&text).map_err(D::Error::custom),
        _ => Err(D::Error::custom(
            "duration must be a string with unit or a millisecond integer",
        )),
    }
}

/// Formats a [`Duration`] as saturated whole milliseconds with an `ms` suffix.
///
/// # Parameters
/// - `duration`: Duration to format.
///
/// # Returns
/// A string in the form `<millis>ms`.
#[inline]
pub fn format(duration: &Duration) -> String {
    format!("{}ms", as_millis_u64(duration))
}

/// Parses a [`Duration`] from a string with a supported unit.
///
/// Bare integers are treated as milliseconds. Supported suffixes are `ns`,
/// `us`, `µs`, `ms`, `s`, `m`, `h`, and `d`.
///
/// # Parameters
/// - `text`: Duration text to parse.
///
/// # Returns
/// The parsed [`Duration`].
///
/// # Errors
/// Returns a message describing invalid syntax, unsupported units, or overflow.
pub fn parse(text: &str) -> Result<Duration, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("duration must not be empty".to_string());
    }
    if let Ok(millis) = trimmed.parse::<u64>() {
        return Ok(Duration::from_millis(millis));
    }

    let (number, unit) = split_number_and_unit(trimmed)?;
    let value = number.parse::<u64>().map_err(|_| {
        format!("invalid duration value `{number}`: expected a non-negative integer")
    })?;

    duration_from_unit(value, unit)
}

/// Splits duration text into integer and unit parts.
///
/// # Parameters
/// - `text`: Non-empty duration text.
///
/// # Returns
/// A tuple containing the number text and unit text.
///
/// # Errors
/// Returns an error when either part is missing.
fn split_number_and_unit(text: &str) -> Result<(&str, &str), String> {
    let split_at = text
        .find(|ch: char| !ch.is_ascii_digit())
        .ok_or_else(|| "duration unit is missing".to_string())?;
    let (number, unit) = text.split_at(split_at);
    if number.is_empty() {
        return Err("duration value is missing".to_string());
    }
    Ok((number, unit))
}

/// Converts an integer value and unit suffix to a [`Duration`].
///
/// # Parameters
/// - `value`: Non-negative integer duration value.
/// - `unit`: Supported unit suffix.
///
/// # Returns
/// The corresponding [`Duration`].
///
/// # Errors
/// Returns an error when the unit is unsupported or the conversion overflows.
fn duration_from_unit(value: u64, unit: &str) -> Result<Duration, String> {
    match unit {
        "ns" => Ok(Duration::from_nanos(value)),
        "us" | "µs" => Ok(Duration::from_micros(value)),
        "ms" => Ok(Duration::from_millis(value)),
        "s" => Ok(Duration::from_secs(value)),
        "m" => value
            .checked_mul(60)
            .map(Duration::from_secs)
            .ok_or_else(|| "duration minutes overflow u64 seconds".to_string()),
        "h" => value
            .checked_mul(60 * 60)
            .map(Duration::from_secs)
            .ok_or_else(|| "duration hours overflow u64 seconds".to_string()),
        "d" => value
            .checked_mul(24 * 60 * 60)
            .map(Duration::from_secs)
            .ok_or_else(|| "duration days overflow u64 seconds".to_string()),
        _ => Err(format!("unsupported duration unit `{unit}`")),
    }
}
