/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Tests for unit-suffixed duration serde adapter.

use std::time::Duration;

use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
    DurationConversionOptions,
    DurationUnit,
};
use qubit_serde::serde::duration_with_unit;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Holder {
    #[serde(with = "qubit_serde::serde::duration_with_unit")]
    duration: Duration,
}

#[test]
fn test_duration_with_unit_serialize_as_ms_string() {
    let holder = Holder {
        duration: Duration::from_millis(1500),
    };

    let json = serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500ms"}"#);
}

#[test]
fn test_duration_with_unit_deserialize_from_supported_units() {
    let cases = [
        ("42", Duration::from_millis(42)),
        ("42ns", Duration::from_nanos(42)),
        ("42us", Duration::from_micros(42)),
        ("42µs", Duration::from_micros(42)),
        ("42μs", Duration::from_micros(42)),
        ("42ms", Duration::from_millis(42)),
        ("42s", Duration::from_secs(42)),
        ("2m", Duration::from_secs(120)),
        ("2h", Duration::from_secs(7200)),
        ("2d", Duration::from_secs(172800)),
    ];

    for (text, expected) in cases {
        let json = format!(r#"{{"duration":"{text}"}}"#);
        let holder: Holder = serde_json::from_str(&json).expect("duration should deserialize");
        assert_eq!(holder.duration, expected);
    }
}

#[test]
fn test_duration_with_unit_deserialize_from_integer_millis() {
    let holder: Holder =
        serde_json::from_str(r#"{"duration":250}"#).expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(250));
}

#[test]
fn test_duration_with_unit_rejects_invalid_unit() {
    let result = serde_json::from_str::<Holder>(r#"{"duration":"250fortnights"}"#);

    assert!(result.is_err());
}

#[test]
fn test_duration_with_unit_format() {
    let text = duration_with_unit::format(&Duration::from_millis(500));

    assert_eq!(text, "500ms");
}

#[test]
fn test_duration_with_unit_format_pins_millisecond_options() {
    let seconds_options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let seconds: String = DataConverter::from(Duration::from_millis(2500))
        .to_with(&seconds_options)
        .expect("duration should format as seconds");
    assert_eq!(seconds, "3s");

    let text = duration_with_unit::format(&Duration::from_millis(2500));

    assert_eq!(text, "2500ms");
}

#[test]
fn test_duration_with_unit_serialize_uses_datatype_rounding() {
    let holder = Holder {
        duration: Duration::from_micros(1500),
    };

    let json = serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"2ms"}"#);
}

#[test]
fn test_duration_with_unit_parse_rejects_empty_text() {
    let result = duration_with_unit::parse(" ");

    assert!(result.is_err());
}

#[test]
fn test_duration_with_unit_parse_pins_millisecond_options_for_bare_numbers() {
    let seconds_options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let seconds: Duration = DataConverter::from("2")
        .to_with(&seconds_options)
        .expect("bare number should parse as seconds");
    assert_eq!(seconds, Duration::from_secs(2));

    let duration = duration_with_unit::parse("2").expect("duration should parse");

    assert_eq!(duration, Duration::from_millis(2));
}

#[test]
fn test_duration_with_unit_deserialize_rejects_invalid_number_and_non_scalar() {
    assert!(serde_json::from_str::<Holder>(r#"{"duration":-1}"#).is_err());
    assert!(serde_json::from_str::<Holder>(r#"{"duration":1.5}"#).is_err());
    for json in [
        r#"{"duration":null}"#,
        r#"{"duration":true}"#,
        r#"{"duration":[]}"#,
        r#"{"duration":{}}"#,
    ] {
        assert!(
            serde_json::from_str::<Holder>(json).is_err(),
            "expected error for {json}"
        );
    }
}

#[test]
fn test_duration_with_unit_parse_errors_and_overflows() {
    let err = duration_with_unit::parse("18446744073709551616000ns").unwrap_err();
    assert!(err.contains("invalid duration value"));

    assert_eq!(
        duration_with_unit::parse("18446744073709551616").unwrap_err(),
        "Conversion error: Cannot convert '18446744073709551616' to Duration: invalid duration value"
    );

    assert_eq!(
        duration_with_unit::parse("x12ms").unwrap_err(),
        "Conversion error: Cannot convert duration string: duration value is missing"
    );

    let vm = u64::MAX / 60 + 1;
    assert!(
        duration_with_unit::parse(&format!("{vm}m"))
            .unwrap_err()
            .contains("minutes overflow")
    );
    let vh = u64::MAX / (60 * 60) + 1;
    assert!(
        duration_with_unit::parse(&format!("{vh}h"))
            .unwrap_err()
            .contains("hours overflow")
    );
    let vd = u64::MAX / (24 * 60 * 60) + 1;
    assert!(
        duration_with_unit::parse(&format!("{vd}d"))
            .unwrap_err()
            .contains("days overflow")
    );
}

#[test]
fn test_duration_with_unit_serialize_function() {
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut buf);
    duration_with_unit::serialize(&Duration::from_millis(7), &mut ser).expect("serialize");
    assert_eq!(String::from_utf8(buf).unwrap(), r#""7ms""#);
}
