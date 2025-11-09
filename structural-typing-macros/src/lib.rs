use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod codegen;
mod parsing;

/// Generates type-safe structural typing support for a struct.
///
/// This macro transforms a struct to track which fields are present/absent at the type level,
/// enabling compile-time enforcement of field requirements.
///
/// **Note**: If you use `#[derive(...)]` on your struct, you must add `derive-where` to your dependencies:
/// ```bash
/// cargo add derive-where
/// ```
///
/// # Generated Code
///
/// For a struct `MyStruct`, this macro generates:
///
/// - A `my_struct` module with `Fields` trait, `FieldSet<...>` type, and `with::` aliases
/// - Builder methods: `.field(value)`
/// - Getter methods: `.get_field()`, `.get_field_mut()`
/// - `.merge()`, `.split()`, `.try_split()` operations
///
/// # Field States
///
/// - **Present**: Value of type `T`
/// - **Optional**: `Option<T>`
/// - **Absent**: `PhantomData<T>`
///
/// # Example
///
/// ```ignore
/// use structural_typing::{structural, presence::Present};
///
/// #[structural]
/// struct User {
///     name: String,
///     email: String,
/// }
///
/// // Build incrementally
/// let user = User::empty().name("Alice".to_owned());
///
/// // Methods can require specific fields
/// impl<F: user::Fields<name = Present>> User<F> {
///     fn greet(&self) -> String {
///         format!("Hello, {}!", self.name)
///     }
/// }
///
/// user.greet(); // ✓ Compiles
/// ```
///
/// # Restrictions
///
/// - Named structs only
/// - No generics
/// - At least one field
#[proc_macro_attribute]
pub fn structural(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match codegen::generate(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
