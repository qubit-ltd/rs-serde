// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the fixed-millisecond unit-suffixed duration serde adapter.

use std::time::Duration;

use qubit_datatype::DurationParseError;
use qubit_serde::serde::duration_millis_with_unit;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Holder {
    #[serde(with = "qubit_serde::serde::duration_millis_with_unit")]
    duration: Duration,
}

#[test]
fn test_duration_millis_with_unit_serialize_as_millisecond_string() {
    let holder = Holder {
        duration: Duration::from_millis(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500ms"}"#);
}

#[test]
fn test_duration_millis_with_unit_serialize_uses_half_up_rounding() {
    let cases = [
        (Duration::from_micros(499), "0ms"),
        (Duration::from_micros(500), "1ms"),
        (Duration::from_micros(1499), "1ms"),
        (Duration::from_micros(1500), "2ms"),
    ];

    for (duration, expected) in cases {
        let holder = Holder { duration };
        let json =
            serde_json::to_value(holder).expect("duration should serialize");
        assert_eq!(json["duration"], expected);
    }
}

#[test]
fn test_duration_millis_with_unit_format_keeps_millisecond_unit() {
    let text = duration_millis_with_unit::format(&Duration::from_millis(2500));

    assert_eq!(text, "2500ms");
}

#[test]
fn test_duration_millis_with_unit_deserialize_supported_input() {
    let holder: Holder = serde_json::from_str(r#"{"duration":"42ns"}"#)
        .expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_nanos(42));
}

#[test]
fn test_duration_millis_with_unit_parse_returns_structured_error() {
    assert_eq!(
        duration_millis_with_unit::parse("12fortnights"),
        Err(DurationParseError::UnsupportedUnit {
            unit: "fortnights".to_string(),
        })
    );
}

#[test]
fn test_duration_millis_with_unit_serialize_function() {
    let mut buffer = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut buffer);
    duration_millis_with_unit::serialize(
        &Duration::from_micros(1500),
        &mut serializer,
    )
    .expect("duration should serialize");

    assert_eq!(
        String::from_utf8(buffer).expect("serialized text should be UTF-8"),
        r#""2ms""#
    );
}
