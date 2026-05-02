/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Serde adapter for [`std::time::Duration`] as a string with a time unit.
//!
//! Serialization emits rounded whole milliseconds with an `ms` suffix.
//! Deserialization accepts strings with `ns`, `us`, `µs`, `μs`, `ms`, `s`,
//! `m`, `h`, or `d` suffixes, and also accepts a bare integer as milliseconds
//! for lenient configuration input.

use std::time::Duration;

use qubit_datatype::DataConverter;
use serde::de::Error;
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

use super::duration_millis::MILLISECOND_CONVERSION_OPTIONS;

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
    let text = format(duration);
    serializer.serialize_str(&text)
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
            DataConverter::from(millis)
                .to_with::<Duration>(&MILLISECOND_CONVERSION_OPTIONS)
                .map_err(D::Error::custom)
        }
        serde_json::Value::String(text) => parse(&text).map_err(D::Error::custom),
        _ => Err(D::Error::custom(
            "duration must be a string with unit or a millisecond integer",
        )),
    }
}

/// Formats a [`Duration`] using explicit millisecond [`DataConverter`] options.
///
/// # Parameters
/// - `duration`: Duration to format.
///
/// # Returns
/// A string in the form `<millis>ms`.
///
#[inline]
pub fn format(duration: &Duration) -> String {
    DataConverter::from(*duration)
        .to_with::<String>(&MILLISECOND_CONVERSION_OPTIONS)
        .expect("Duration to String conversion should be infallible")
}

/// Parses a [`Duration`] from a string with a supported unit.
///
/// Bare integers are treated as milliseconds. Supported suffixes are `ns`,
/// `us`, `µs`, `μs`, `ms`, `s`, `m`, `h`, and `d`.
///
/// # Parameters
/// - `text`: Duration text to parse.
///
/// # Returns
/// The parsed [`Duration`].
///
/// # Errors
/// Returns a message describing invalid syntax, unsupported units, or overflow.
#[inline]
pub fn parse(text: &str) -> Result<Duration, String> {
    DataConverter::from(text)
        .to_with::<Duration>(&MILLISECOND_CONVERSION_OPTIONS)
        .map_err(|error| error.to_string())
}
