//! # Structural Typing for Rust
//!
//! Track which struct fields are present/absent at the type level.
//!
//! ## Quick Example
//!
//! ```
//! use structural_typing::{structural, presence::Present};
//!
//! #[structural]
//! struct User { name: String, email: String }
//!
//! // Build incrementally - type tracks which fields are set
//! let user = User::empty().name("Alice".into());
//!
//! // Methods can require specific fields
//! impl<F: user::Fields<name = Present>> User<F> {
//!     fn greet(&self) -> String {
//!         format!("Hello, {}!", self.name)
//!     }
//! }
//!
//! user.greet(); // ✓ Compiles - name is Present
//! // User::empty().greet(); // ✗ Compile error - name is Absent
//! ```
//!
//! ## Core Concepts
//!
//! - **Field States**: `Present` (has value), `Optional` (has `Option<T>`), `Absent` (no value)
//! - **Builder API**: `.field(val)`, `.maybe_field(opt)`, `.unset_field()`
//! - **Type Algebra**: `select!(name, ?email)` and `modify!(AllAbsent, +name)`
//! - **Bounded Impls**: Methods requiring specific fields via trait bounds
//!
//! See [`examples/user_fields.rs`](https://github.com/Crazytieguy/structural-typing-rs/blob/master/structural-typing/examples/user_fields.rs) for comprehensive usage.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

/// Runtime field access trait for checking presence and getting values.
pub mod access;
/// Type-level presence markers and traits for field state tracking.
pub mod presence;
/// Traits for splitting structs into selected fields and remainder.
pub mod split;

pub use structural_typing_macros::structural;
