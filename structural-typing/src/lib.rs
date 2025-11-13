//! # Structural Typing for Rust
//!
//! Structural typing separates schemas (what fields can exist) from selections (what fields must be
//! present). Optionality is context-dependent—a name is always a string, but different contexts may
//! or may not require it. Instead of creating separate types for each context (`CreateUser`,
//! `UpdateUser`, `DBUser`...) or using `Option<T>` everywhere, define your schema once and use
//! generic trait bounds for context-specific requirements—all verified at compile time.
//!
//! This design is inspired by Rich Hickey's [`Maybe Not`](https://www.youtube.com/watch?v=YR5WdGrpoug)
//! talk, which articulates the principle of separating attribute definitions from their use in aggregates,
//! along with ideas from TypeScript's structural types and RDF's independent attributes.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! structural-typing = "0.1.7"
//! derive-where = "1.6"  # For deriving traits on structural types
//! ```
//!
//! ## Usage
//!
//! ### Define the schema
//!
//! ```
//! use structural_typing::structural;
//!
//! #[structural]
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//! }
//! ```
//!
//! The `#[structural]` macro generates a module named `user` containing a `Fields` trait with
//! associated types for each field. This enables compile-time tracking of which fields are present.
//!
//! ### Generic functions with field requirements
//!
//! Functions can require specific fields through trait bounds using the generated `Fields` trait.
//! The `Presence` type can be `Present` (`T`), `Optional` (`Option<T>`), or `Absent` (`PhantomData<T>`):
//!
//! ```
//! # use structural_typing::{structural, presence::Present};
//! #
//! # #[structural]
//! # struct User {
//! #     id: u32,
//! #     name: String,
//! #     email: String,
//! # }
//! #
//! // Requires both id and name
//! fn display_user<F: user::Fields<id = Present, name = Present>>(user: &User<F>) {
//!     println!("User #{}: {}", user.id, user.name);
//! }
//! ```
//!
//! Relaxing requirements is backward compatible. Existing callers with id and name continue to work
//! if we remove the `name` requirement—unlike changing a function parameter from `T` to `Option<T>`,
//! which breaks all existing call sites.
//!
//! ```
//! # use structural_typing::{structural, presence::Present};
//! #
//! # #[structural]
//! # struct User {
//! #     id: u32,
//! #     name: String,
//! #     email: String,
//! # }
//! #
//! // Requires only id; adapts behavior based on whether name is present
//! fn display_user<F: user::Fields<id = Present>>(user: &User<F>) {
//!     if let Some(name) = user.get_name() {
//!         println!("User #{}: {}", user.id, name);
//!     } else {
//!         println!("User #{}", user.id);
//!     }
//! }
//! ```
//!
//! ### Build instances incrementally
//!
//! The builder API infers field presence from the value type:
//!
//! ```
//! # use structural_typing::{structural, presence::Present};
//! #
//! # #[structural]
//! # struct User {
//! #     id: u32,
//! #     name: String,
//! #     email: String,
//! # }
//! #
//! # fn display_user<F: user::Fields<id = Present>>(user: &User<F>) {
//! #     if let Some(name) = user.get_name() {
//! #         println!("User #{}: {}", user.id, name);
//! #     } else {
//! #         println!("User #{}", user.id);
//! #     }
//! # }
//! #
//! let bob = User::empty().id(123);
//! display_user(&bob);
//!
//! let bob = bob.name("Bob".to_owned());
//! ```
//!
//! ### Serde integration
//!
//! Enable the `serde` feature to use structural types with serde.
//!
//! The `select!` macro creates concrete types, useful for serialization boundaries:
//!
//! ```ignore
//! use serde::{Deserialize, Serialize};
//!
//! #[structural]
//! #[derive(Serialize, Deserialize)]
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//! }
//!
//! let json = r#"{"name": "Alice", "email": "alice@example.com"}"#;
//! // name optional, email present, id absent
//! let alice: User<select!(user: ?name, email)> = serde_json::from_str(json)?;
//!
//! let response = alice.id(42);
//! let json = serde_json::to_string(&response)?;
//! // => {"id":42, "name": "Alice", "email": "alice@example.com"}
//! ```
//!
//! ### Extract and merge
//!
//! ```
//! # use structural_typing::{structural, select};
//! #
//! # #[structural]
//! # struct User {
//! #     id: u32,
//! #     name: String,
//! #     email: String,
//! # }
//! #
//! # let alice = User::empty()
//! #     .id(42)
//! #     .name("Alice".to_owned())
//! #     .email("alice@example.com".to_owned());
//! #
//! // Extract name and email, get back credentials and remainder with just id
//! let (credentials, id_only) = alice.extract::<select!(user: name, email)>();
//! assert_eq!(credentials.name, "Alice");
//! assert_eq!(id_only.id, 42);
//! ```
//!
//! Merge instances:
//!
//! ```
//! # use structural_typing::structural;
//! #
//! # #[structural]
//! # struct User {
//! #     id: u32,
//! #     name: String,
//! #     email: String,
//! # }
//! #
//! # let credentials = User::empty()
//! #     .name("Alice".to_owned())
//! #     .email("alice@example.com".to_owned());
//! # let id_only = User::empty().id(42);
//! #
//! let alice = credentials.merge(id_only);
//! // merged values override existing values
//! let overridden = alice.merge(User::empty().id(21));
//! assert_eq!(overridden.name, "Alice");
//! assert_eq!(overridden.id, 21);
//! ```
//!
//! ### Nested schemas
//!
//! Schemas can contain other structural types:
//!
//! ```ignore
//! use structural_typing::{structural, presence::Present};
//!
//! #[structural]
//! struct Address {
//!     street: String,
//!     city: String,
//!     zip: String,
//! }
//!
//! #[structural]
//! struct User<A: address::Fields> {
//!     id: u32,
//!     name: String,
//!     address: Address<A>,
//! }
//!
//! // Require specific nested fields
//! fn ship<A, F>(user: User<A, F>)
//! where
//!     A: address::Fields<street = Present, city = Present>,
//!     F: user::Fields<id = Present, address = Present>
//! {
//!     println!("Shipping to {} at: {}, {}",
//!         user.id, user.address.street, user.address.city);
//! }
//! ```
//!
//! See the [examples](https://github.com/Crazytieguy/structural-typing-rs/tree/master/examples)
//! directory for more comprehensive usage.
//!
//! ## Design
//!
//! The `#[structural]` macro generates a module containing a `Fields` trait with associated types
//! for each field. Field presence is tracked through type-level markers (`Present`, `Optional`,
//! `Absent`) that implement the `Presence` trait. Functions express requirements through trait bounds,
//! and the builder API uses type inference to determine field states.
//!
//! This approach enables:
//! - **Compile-time verification**: All field requirements checked during compilation
//! - **Zero runtime overhead**: No dynamic checks or boxing
//! - **API evolution**: Relaxing requirements is backward compatible
//! - **Flexible composition**: Merge, extract, and transform with compile-time guarantees

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
