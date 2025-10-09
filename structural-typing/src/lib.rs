#![warn(clippy::pedantic)]
#![deny(missing_docs)]

//! # Structural Typing
//!
//! A Rust library for compile-time structural typing with type-state pattern.
//!
//! This library allows you to create structs with optionally-present fields that are tracked
//! at compile-time using Rust's type system. It provides a type-safe way to build up structs
//! incrementally, similar to the builder pattern but with stronger guarantees.
//!
//! ## Example
//!
//! ```rust
//! use structural_typing::structural;
//!
//! #[structural]
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! let person = Person::empty()
//!     .name("Alice".into())
//!     .age(30);
//!
//! // Fields are directly accessible when known to be present
//! assert_eq!(person.name, "Alice");
//! assert_eq!(person.age, 30);
//! ```

pub use structural_typing_macros::structural;

/// Types and traits for tracking field presence at compile-time.
pub mod presence;

/// Traits for accessing fields regardless of their presence state.
pub mod access;

/// Conversions between different presence states.
pub mod presence_convert;

pub use presence::{Present, Optional, Absent, Presence};
pub use access::Access;
pub use presence_convert::PresenceConvert;

#[doc(hidden)]
pub mod __private {
    pub use core::marker::PhantomData;
    pub use core::option::Option;
    pub use core::fmt;
    pub use core::clone::Clone;
}
