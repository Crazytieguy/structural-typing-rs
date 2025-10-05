#![warn(clippy::pedantic)]
#![deny(missing_docs)]

//! Procedural macros for structural typing.
//!
//! This crate provides the `#[structural]` attribute macro used by the `structural-typing` crate.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod codegen;
mod parsing;
mod derive;

/// Attribute macro for creating structurally-typed structs.
///
/// This macro transforms a regular struct into one with compile-time tracked field presence.
/// It generates:
/// - A state module with presence tracking types
/// - Setter methods that transition between states
/// - An `empty()` constructor (requires always-present fields as parameters)
/// - `merge()` functionality to combine partial structs
/// - `require_*()` methods for runtime presence checking
///
/// # Examples
///
/// Basic usage:
/// ```ignore
/// use structural_typing::structural;
///
/// #[structural]
/// struct Person {
///     name: String,
///     age: u8,
/// }
///
/// let person = Person::empty()
///     .name("Alice".into())
///     .age(30);
/// ```
///
/// With always-present fields:
/// ```ignore
/// use structural_typing::structural;
///
/// #[structural]
/// struct Config {
///     #[always]
///     version: u32,
///     name: String,
/// }
///
/// // version must be provided to empty()
/// let config = Config::empty(1).name("app".into());
/// ```
#[proc_macro_attribute]
pub fn structural(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match codegen::generate_structural(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
