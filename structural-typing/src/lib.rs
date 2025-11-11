//! # Structural Typing for Rust
//!
//! Define a struct once and use it with different field combinations, tracked at compile time.
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
//! struct User { id: u32, name: String, email: String }
//!
//! type Create = select!(user: name, email);
//!
//! fn create_user(data: User<Create>) -> User {
//!     data.id(generate_id())
//! }
//!
//! fn update_user<F: user::Fields<id = Present>>(data: User<F>) {
//!     if let Some(name) = data.get_name() {
//!         update_in_db(data.id, name);
//!     }
//! }
//! # fn generate_id() -> u32 { 42 }
//! # fn update_in_db(_id: u32, _name: &String) {}
//! ```
//!
//! ## Core Concepts
//!
//! - **Field States**: `Present`, `Optional`, `Absent`
//! - **Builder API**: `.field(value)` infers presence from type
//! - **Type Selection**: `select!` macro or type aliases
//! - **Bounded Impls**: Require specific fields via trait bounds
//! - **Merge and Extract**: Combine partial structs or extract field subsets
//!
//! For serde integration and other details, see the [`structural`] macro documentation.
//!
//! See the [examples](https://github.com/Crazytieguy/structural-typing-rs/tree/master/examples) directory for comprehensive usage.
//!
//! ## Constraints
//!
//! - Named structs only (not tuple structs or enums)
//! - No generic parameters
//! - At least one field required

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

/// Runtime field access trait for checking presence and getting values.
pub mod access;
/// Type-level presence markers and traits for field state tracking.
pub mod presence;
/// Traits for splitting structs into selected fields and remainder.
pub mod extract;

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
