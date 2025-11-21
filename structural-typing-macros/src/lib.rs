use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod analysis;
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
/// - `.merge()`, `.extract()`, `.try_extract()` operations
///
/// Supports user-defined generic type parameters and nested structural types (see `examples/nested.rs`).
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
/// # Serde Support
///
/// Deserialization uses a helper struct where all fields are `Option<T>` with `#[serde(default)]`,
/// then converts via `TryFrom`. Present fields error if missing, Optional fields become `Some`/`None`,
/// Absent fields are ignored.
///
/// **Limitations:**
/// - Present fields cannot use the field type's `Default` impl during deserialization
/// - Optional fields cannot distinguish between missing and explicit `null` in JSON
/// - Absent fields silently ignore data if present in input
/// - All field types must implement `Deserialize` even if marked Absent
///
/// **Custom deserializers:** If using `#[serde(deserialize_with)]`, the function must produce `Option<T>`.
///
/// **Incompatible:** `#[serde(default)]`, `#[serde(skip)]`, `#[serde(skip_deserializing)]`, `#[serde(flatten)]`.
/// **Compatible:** `rename`, `alias`, `rename_all`, `deserialize_with` (with `Option<T>` output).
///
/// # Restrictions
///
/// - Named structs only
/// - At least one field
#[proc_macro_attribute]
pub fn structural(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match codegen::generate(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
