/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Serde adapters for common standard-library and Qubit value types.
//!
//! The modules in this namespace are intended for use with
//! `#[serde(with = "...")]` on fields that need a stable interchange format.

pub mod duration_millis;
pub mod duration_with_unit;
