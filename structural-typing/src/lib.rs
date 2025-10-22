//! # Structural Typing for Rust
//!
//! Define structs with optional fields, tracked at the type level.
//!
//! See `examples/user_fields.rs` for comprehensive usage.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

/// Runtime field access trait for checking presence and getting values.
pub mod access;
/// Type-level presence markers and traits for field state tracking.
pub mod presence;

pub use structural_typing_macros::structural;
