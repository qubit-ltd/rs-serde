// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Serde
//!
//! Provides reusable serde adapters and helper functions.

#[path = "serde/mod.rs"]
mod serde_impl;

/// Serde adapters for common standard-library and Qubit value types.
///
/// Use these modules with `#[serde(with = "...")]` when a field requires a
/// stable interchange format.
pub mod serde {
    pub use super::serde_impl::{
        duration_millis,
        duration_millis_with_unit,
        duration_with_unit,
    };
}
