//! # Structural Typing for Rust
//!
//! Track which struct fields are present/absent at the type level.
//!
//! ## Installation
//!
//! ```bash
//! cargo add structural-typing
//! ```
//!
//! If using derives on `#[structural]` structs, also add:
//! ```bash
//! cargo add derive-where
//! ```
//!
//! ## Quick Example
//!
//! ```
//! use structural_typing::{structural, presence::Present, select};
//!
//! #[structural]
//! struct User { name: String, email: String }
//!
//! // Build incrementally - type tracks which fields are set
//! let user = User::empty().name("Alice".to_owned());
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
//!
//! // Use select! to create field sets
//! type NameAndEmail = select!(user: name, email);
//! ```
//!
//! ## Core Concepts
//!
//! - **Field States**: `Present` (has value), `Optional` (has `Option<T>`), `Absent` (no value)
//! - **Builder API**: `.field(value)` infers presence from type (`T` → `Present`, `Option<T>` → `Optional`, `PhantomData<T>` → Absent)
//! - **Type Selection**: `select!(module: field1, ?field2)` macro or direct type aliases
//! - **Bounded Impls**: Methods requiring specific fields via trait bounds
//!
//! See the [examples](https://github.com/Crazytieguy/structural-typing-rs/tree/master/examples) directory for comprehensive usage including merge, split, serde integration, and more.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

/// Runtime field access trait for checking presence and getting values.
pub mod access;
/// Type-level presence markers and traits for field state tracking.
pub mod presence;
/// Traits for splitting structs into selected fields and remainder.
pub mod split;

pub use structural_typing_macros::structural;

/// Construct a `FieldSet` by selecting fields from a module.
///
/// # Syntax
/// - `select!(module: field)` - field is Present
/// - `select!(module: ?field)` - field is Optional
/// - `select!(module: ?all)` - all fields Optional
/// - Multiple fields compose: `select!(module: field1, ?field2)` expands to `module::with::field1::Present<module::with::field2::Optional>`
///
/// # Examples
/// ```ignore
/// type Create = select!(user: name, email);
/// type Update = select!(user: ?name, ?email);
/// fn create_user(user: User<Create>) {...}
/// ```
#[macro_export]
macro_rules! select {
    // Error: invalid + prefix
    ($($module:ident)::+ : + $field:ident $($rest:tt)*) => {
        compile_error!("Invalid prefix '+' in select!. Use 'field' for Present or '?field' for Optional")
    };

    // Error: invalid - prefix
    ($($module:ident)::+ : - $field:ident $($rest:tt)*) => {
        compile_error!("Invalid prefix '-' in select!. Use 'field' for Present or '?field' for Optional")
    };

    // Error: empty field list
    ($($module:ident)::+ :) => {
        compile_error!("select! requires at least one field")
    };

    // Single field: Present
    ($($module:ident)::+ : $field:ident) => {
        $($module)::+::with::$field::Present
    };

    // Single field: Present with trailing comma
    ($($module:ident)::+ : $field:ident,) => {
        $($module)::+::with::$field::Present
    };

    // Single field: Optional
    ($($module:ident)::+ : ? $field:ident) => {
        $($module)::+::with::$field::Optional
    };

    // Single field: Optional with trailing comma
    ($($module:ident)::+ : ? $field:ident,) => {
        $($module)::+::with::$field::Optional
    };

    // Multiple fields: Present + rest
    ($($module:ident)::+ : $field:ident, $($rest:tt)+) => {
        $($module)::+::with::$field::Present<select!($($module)::+ : $($rest)+)>
    };

    // Multiple fields: Optional + rest
    ($($module:ident)::+ : ? $field:ident, $($rest:tt)+) => {
        $($module)::+::with::$field::Optional<select!($($module)::+ : $($rest)+)>
    };
}
