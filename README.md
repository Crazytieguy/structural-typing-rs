# Structural Typing for Rust

Structural typing allows you to define structs where fields can be optionally present, tracked at compile-time using Rust's type system. This enables you to write methods that only work when certain fields are available, all verified at compile time with no runtime overhead.

The main inspirations for this are TypeScript's structural typing, RDF, and [this talk](https://www.youtube.com/watch?v=YR5WdGrpoug&list=PLZdCLR02grLrEwKaZv-5QbUzK0zGKOOcr&index=2&t=9s).

## Features

- **Type-safe structural typing**: Fields can be `Present`, `Optional`, or `Absent`, tracked at compile time
- **Builder-like API**: Set fields using method chaining, with each setter returning a new type
- **Conditional methods**: Implement methods that only work when specific fields are present
- **Always-present fields**: Mark fields as required with `#[always]`
- **Merge support**: Combine partial structs, with later values taking precedence
- **Derive support**: Automatically implements `Debug`, `Clone`, and other standard traits
- **Zero runtime cost**: All field presence is tracked via types and compiled away
- **Clean syntax**: Uses proc macros for ergonomic struct definitions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
structural-typing = "0.2"
```

## Quick Start

```rust
use structural_typing::{structural, Present};

#[structural]
struct Person {
    name: String,
    age: u8,
    height: f32,
}

// Method that only requires a name
impl<S> Person<S>
where
    S: person_state::State<Name = Present>,
{
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

// Method that requires both name and age
impl<S> Person<S>
where
    S: person_state::State<Name = Present, Age = Present>,
{
    fn introduce(&self) -> String {
        format!("I'm {} and I'm {} years old", self.name, self.age)
    }
}

fn main() {
    // Start with an empty person
    let person = Person::empty();

    // Add a name - now we can call greet()
    let person = person.name("Alice".into());
    println!("{}", person.greet()); // OK!
    // person.introduce(); // ERROR: age not set yet

    // Add age - now we can call introduce()
    let person = person.age(30);
    println!("{}", person.introduce()); // OK!
}
```

## How It Works

The `#[structural]` macro generates:

1. **The struct** with a state parameter tracking field presence
2. **A state module** with types representing which fields are present
3. **Methods** for setting fields, merging, and runtime checks

### Field Access

Fields have different types depending on their presence state:

- **Present**: Direct access (`self.name`)
- **Absent**: `PhantomData<T>` (zero-size, can't be accessed)
- **Optional**: `Option<T>`

The `Access` trait provides uniform access regardless of state:

```rust
use structural_typing::{structural, Access};

#[structural]
struct Person {
    name: String,
    age: u8,
}

let person = Person::empty().name("Bob".into());

person.name.get();  // Some(&"Bob")
person.age.get();   // None (age is absent)
```

**Note:** You must import the `Access` trait to use `.get()`, `.get_mut()`, and `.remove()` methods.

### Always-Present Fields

You can mark fields as always present using the `#[always]` attribute. These fields must be provided when creating the struct and are never absent:

```rust
use structural_typing::structural;

#[structural]
struct Config {
    #[always]
    version: u32,  // Always required
    name: String,  // Optional
}

let config = Config::empty(1); // version is required
let config = config.name("app".into());
```

### Derive Support

The `#[structural]` macro automatically supports `#[derive(Debug, Clone)]` and will generate appropriate implementations:

```rust
#[structural]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

let person = Person::empty().name("Alice".into());
println!("{:?}", person);  // Debug works
let cloned = person.clone();  // Clone works
```

### Merging

Combine partial structs, with the second struct's fields taking precedence:

```rust
let p1 = Person::empty().name("Alice".into());
let p2 = Person::empty().age(30);

let merged = p1.merge(p2);
// merged has name="Alice" and age=30
```

### Runtime Checks

Use `require_*` methods to promote optional/absent fields to present at runtime:

```rust
let person = Person::empty();

if let Some(person) = person.require_name() {
    // Now person has Name = Present
    println!("{}", person.greet());
}
```

## Examples

See the [examples](structural-typing/examples/) directory for more:

- [`basic.rs`](structural-typing/examples/basic.rs) - Basic usage and merging
- [`methods.rs`](structural-typing/examples/methods.rs) - Conditional method implementations

## Inspiration

This project draws inspiration from:

- **TypeScript**: Structural type system and optional properties
- **RDF**: Flexible data modeling with optional fields
- **Rust builder patterns**: Type-state pattern for compile-time validation
- **[bon](https://github.com/elastio/bon)**: Proc macro patterns and type-state builders

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
