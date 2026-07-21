// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for unit-suffixed duration serde adapter.

use std::time::Duration;

use qubit_datatype::DurationParseError;
use qubit_serde::serde::duration_with_unit;
use serde::de::value::{
    Error as ValueError,
    I128Deserializer,
    I64Deserializer,
    StringDeserializer,
    U128Deserializer,
};
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
fn test_duration_with_unit_serialize_as_exact_string() {
    let holder = Holder {
        duration: Duration::from_millis(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500ms"}"#);
}

#[test]
fn test_duration_with_unit_deserialize_from_supported_units() {
    let cases = [
        ("42ns", Duration::from_nanos(42)),
        ("42us", Duration::from_micros(42)),
        ("42µs", Duration::from_micros(42)),
        ("42μs", Duration::from_micros(42)),
        ("42ms", Duration::from_millis(42)),
        ("42s", Duration::from_secs(42)),
        ("2min", Duration::from_secs(120)),
        ("2h", Duration::from_secs(7200)),
        ("2d", Duration::from_secs(172800)),
    ];

    for (text, expected) in cases {
        let json = format!(r#"{{"duration":"{text}"}}"#);
        let holder: Holder =
            serde_json::from_str(&json).expect("duration should deserialize");
        assert_eq!(holder.duration, expected);
    }

    for unsupported in ["42m"] {
        let json = format!(r#"{{"duration":"{unsupported}"}}"#);
        assert!(serde_json::from_str::<Holder>(&json).is_err());
    }
}

#[test]
fn test_duration_with_unit_deserialize_from_integer_millis() {
    assert!(serde_json::from_str::<Holder>(r#"{"duration":250}"#).is_err());
}

#[test]
fn test_duration_with_unit_deserialize_from_owned_string() {
    let deserializer =
        StringDeserializer::<ValueError>::new("42ns".to_string());
    let duration = duration_with_unit::deserialize(deserializer)
        .expect("owned duration text should deserialize");

    assert_eq!(duration, Duration::from_nanos(42));
}

#[test]
fn test_duration_with_unit_deserialize_from_wide_unsigned_integer() {
    let millis = u128::from(u64::MAX) * 1_000 + 999;
    let deserializer = U128Deserializer::<ValueError>::new(millis);
    assert!(duration_with_unit::deserialize(deserializer).is_err());
}

#[test]
fn test_duration_with_unit_deserialize_from_signed_integers() {
    let i64_deserializer = I64Deserializer::<ValueError>::new(250);
    assert!(duration_with_unit::deserialize(i64_deserializer).is_err());
    let i128_deserializer = I128Deserializer::<ValueError>::new(500);
    assert!(duration_with_unit::deserialize(i128_deserializer).is_err());
}

#[test]
fn test_duration_with_unit_deserialize_rejects_negative_signed_integers() {
    let i64_result =
        duration_with_unit::deserialize(I64Deserializer::<ValueError>::new(-1));
    let i128_result = duration_with_unit::deserialize(I128Deserializer::<
        ValueError,
    >::new(-1));

    assert!(i64_result.is_err());
    assert!(i128_result.is_err());
}

#[test]
fn test_duration_with_unit_deserialize_rejects_wide_integer_overflow() {
    let millis = (u128::from(u64::MAX) + 1) * 1_000;
    let result = duration_with_unit::deserialize(
        U128Deserializer::<ValueError>::new(millis),
    );

    assert!(result.is_err());
}

#[test]
fn test_duration_with_unit_rejects_invalid_unit() {
    let result =
        serde_json::from_str::<Holder>(r#"{"duration":"250fortnights"}"#);

    assert!(result.is_err());
}

#[test]
fn test_duration_with_unit_format() {
    let text = duration_with_unit::format(&Duration::from_millis(500));

    assert_eq!(text, "500ms");
}

#[test]
fn test_duration_with_unit_format_selects_largest_exact_unit() {
    let cases = [
        (Duration::ZERO, "0ms"),
        (Duration::from_secs(2 * 24 * 60 * 60), "2d"),
        (Duration::from_secs(2 * 60 * 60), "2h"),
        (Duration::from_secs(2 * 60), "2min"),
        (Duration::from_secs(42), "42s"),
        (Duration::from_millis(2500), "2500ms"),
        (Duration::from_micros(500), "500µs"),
        (Duration::from_nanos(42), "42ns"),
    ];

    for (duration, expected) in cases {
        assert_eq!(duration_with_unit::format(&duration), expected);
    }
}

#[test]
fn test_duration_with_unit_serialize_preserves_sub_millisecond_precision() {
    let holder = Holder {
        duration: Duration::from_micros(1500),
    };

    let json =
        serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":"1500µs"}"#);
}

#[test]
fn test_duration_with_unit_format_round_trips_duration_max() {
    let text = duration_with_unit::format(&Duration::MAX);
    let parsed = duration_with_unit::parse(&text)
        .expect("formatted maximum should parse");

    assert_eq!(parsed, Duration::MAX);
}

#[test]
fn test_duration_with_unit_format_round_trips_semantic_boundaries() {
    let seconds = [
        0,
        1,
        59,
        60,
        3599,
        3600,
        86_399,
        86_400,
        u64::from(u32::MAX),
        u64::MAX,
    ];
    let nanoseconds = [
        0,
        1,
        999,
        1_000,
        999_999,
        1_000_000,
        499_999_999,
        500_000_000,
        999_999_999,
    ];

    for seconds in seconds {
        for nanoseconds in nanoseconds {
            let duration = Duration::new(seconds, nanoseconds);
            let text = duration_with_unit::format(&duration);
            let parsed = duration_with_unit::parse(&text)
                .expect("formatted boundary duration should parse");
            assert_eq!(parsed, duration, "failed to round-trip {text}");
        }
    }
}

#[test]
fn test_duration_with_unit_parse_rejects_empty_text() {
    let result = duration_with_unit::parse(" ");

    assert!(result.is_err());
}

/// Test duration text is not implicitly trimmed by the adapter.
#[test]
fn test_duration_with_unit_parse_rejects_surrounding_whitespace() {
    assert!(duration_with_unit::parse(" 2ms ").is_err());
    assert!(serde_json::from_str::<Holder>(r#"{"duration":" 2ms "}"#).is_err());
}

#[test]
fn test_duration_with_unit_parse_rejects_bare_numbers() {
    assert!(duration_with_unit::parse("2").is_err());
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
    assert!(duration_with_unit::parse("18446744073709551616000ns").is_ok());
    assert!(duration_with_unit::parse("18446744073709551616").is_err());

    assert_eq!(
        duration_with_unit::parse("340282366920938463463374607431768211456ns"),
        Err(DurationParseError::OutOfRange)
    );
    assert_eq!(
        duration_with_unit::parse(&format!("{}s", u128::MAX)),
        Err(DurationParseError::OutOfRange)
    );
    assert_eq!(
        duration_with_unit::parse("x12ms"),
        Err(DurationParseError::InvalidSyntax)
    );
    assert_eq!(
        duration_with_unit::parse("12.5s"),
        Err(DurationParseError::InvalidSyntax)
    );
    assert_eq!(
        duration_with_unit::parse("12fortnights"),
        Err(DurationParseError::UnsupportedUnit {
            unit: "fortnights".to_string(),
        })
    );

    let vm = u64::MAX / 60 + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vm}min")),
        Err(DurationParseError::OutOfRange)
    );
    let vh = u64::MAX / (60 * 60) + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vh}h")),
        Err(DurationParseError::OutOfRange)
    );
    let vd = u64::MAX / (24 * 60 * 60) + 1;
    assert_eq!(
        duration_with_unit::parse(&format!("{vd}d")),
        Err(DurationParseError::OutOfRange)
    );
}

#[test]
fn test_duration_with_unit_serialize_function() {
    let mut buf = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut buf);
    duration_with_unit::serialize(&Duration::from_millis(7), &mut ser)
        .expect("serialize");
    assert_eq!(
        String::from_utf8(buf).expect("serialized text should be UTF-8"),
        r#""7ms""#
    );
}

#[test]
fn test_duration_with_unit_postcard_round_trip() {
    let holder = Holder {
        duration: Duration::from_nanos(42),
    };
    let bytes =
        postcard::to_stdvec(&holder).expect("duration should serialize");
    let decoded: Holder =
        postcard::from_bytes(&bytes).expect("duration should deserialize");

    assert_eq!(decoded, holder);
}
