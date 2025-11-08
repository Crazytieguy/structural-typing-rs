use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod codegen;
mod parsing;

/// Generates type-safe structural typing support for a struct.
///
/// This macro transforms a struct to track which fields are present/absent at the type level,
/// enabling compile-time enforcement of field requirements.
///
/// # Generated Code
///
/// For a struct `MyStruct`, this macro generates:
///
/// - A `my_struct` module containing:
///   - `Fields` trait - for type-level field requirements
///   - `FieldSet<...>` - zero-cost marker type tracking field presence
///   - Type aliases in `with::` module for field presence combinations
/// - Builder methods: `.field(value)` that infers presence from value type
/// - Getter methods: `.get_field()`, `.get_field_mut()`
/// - `.merge()` - combine two partial structs
/// - `.split()` / `.try_split()` - split into selected fields and remainder
///
/// # Field States
///
/// - **Present**: Field has a value of type `T`
/// - **Optional**: Field has type `Option<T>`
/// - **Absent**: Field is `PhantomData<T>` (zero cost)
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
/// - Only named structs supported (not tuple structs or enums)
/// - No generic parameters allowed
/// - Struct must have at least one field
#[proc_macro_attribute]
pub fn structural(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match codegen::generate(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
