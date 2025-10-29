# Structural Typing for Rust

Define structs with optional fields, tracked at the type level. Inspired by TypeScript, RDF, and [this talk](https://www.youtube.com/watch?v=YR5WdGrpoug&list=PLZdCLR02grLrEwKaZv-5QbUzK0zGKOOcr&index=2&t=9s).

## Example

```rust
use structural_typing::{structural, presence::Present};

#[structural]
#[derive(Clone, Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

fn main() {
    // Build with some fields
    let user = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into());

    // This compiles - greet() requires name field
    println!("{}", user.greet());

    let user = user.age(30);
    // Now all fields present
}

// Methods that require specific fields
impl<F: user::Fields<name = Present>> User<F> {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
```

See [examples/user_fields.rs](structural-typing/examples/user_fields.rs) for comprehensive usage including merge, projection, serde integration, and more.

## Features

- Type-level field tracking (`Present`, `Optional`, `Absent`)
- Compile-time enforcement of field requirements
- Builder pattern with `.field()`, `.maybe_field()`, `.unset_field()`
- Type-safe merge and projection
- Automatic serde support
- Zero runtime overhead

## Status

Experimental. API subject to change.

## License

MIT OR Apache-2.0
