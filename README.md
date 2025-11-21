# Structural Typing for Rust

**Status**: Experimental. API subject to change.

## Overview

Structural typing separates schemas (what fields can exist) from selections (what fields must be present). Optionality is context-dependent—a name is always a string, but different contexts may or may not require it. Instead of creating separate types for each context (`CreateUser`, `UpdateUser`, `DBUser`...) or using `Option<T>` everywhere, define your schema once and use generic trait bounds for context-specific requirements—all verified at compile time.

This design is inspired by Rich Hickey's [Maybe Not](https://www.youtube.com/watch?v=YR5WdGrpoug) talk, which articulates the principle of separating attribute definitions from their use in aggregates, along with ideas from TypeScript's structural types and RDF's independent attributes.

## Installation

```toml
[dependencies]
structural-typing = "0.1.7"
derive-where = "1.6"  # For deriving traits on structural types
```

## Usage

### Define the schema

```rust
use structural_typing::structural;

#[structural]
struct User {
    id: u32,
    name: String,
    email: String,
}
```

The `#[structural]` macro generates a module named `user` containing a `Fields` trait with associated types for each field. This enables compile-time tracking of which fields are present.

### Generic functions with field requirements

Functions can require specific fields through trait bounds using the generated `Fields` trait. The `Presence` type can be `Present` (`T`), `Optional` (`Option<T>`), or `Absent` (`PhantomData<T>`):

```rust
use structural_typing::presence::Present;

// Requires both id and name
fn display_user<F: user::Fields<id = Present, name = Present>>(user: &User<F>) {
    println!("User #{}: {}", user.id, user.name);
}
```

Relaxing requirements is backward compatible. Existing callers with id and name continue to work if we remove the `name` requirement—unlike changing a function parameter from `T` to `Option<T>`, which breaks all existing call sites.

```rust
use structural_typing::access::Access;

// Requires only id; adapts behavior based on whether name is present
fn display_user<F: user::Fields<id = Present>>(user: &User<F>) {
    if let Some(name) = user.name.get() {
        println!("User #{}: {}", user.id, name);
    } else {
        println!("User #{}", user.id);
    }
}
```

### Build instances incrementally

The builder API infers field presence from the value type:

```rust
let bob = User::empty().id(123);
display_user(&bob);

let bob = bob.name("Bob".to_owned());
```

### Serde integration

Enable the `serde` feature to use structural types with serde.

The `select!` macro creates concrete types, useful for serialization boundaries:

```rust
use serde::{Deserialize, Serialize};

#[structural]
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn insert_user<F: user::Fields<email = Present>>(
    user: User<F>
) -> User<select!(user: id, ..F)> {
    // Insert to database, get generated ID
    user.id(42)
}

let json = r#"{"name": "Alice", "email": "alice@example.com"}"#;
// name optional, email present, id absent
let req: User<select!(user: name?, email)> = serde_json::from_str(json)?;

let response = insert_user(req);
let json = serde_json::to_string(&response)?;
// => {"id":42, "name": "Alice", "email": "alice@example.com"}
```

### Extract and merge

```rust
use structural_typing::select;

let (credentials, id_only) = alice.extract::<select!(user: name, email)>();
assert_eq!(credentials.name, "Alice");
assert_eq!(id_only.id, 42);

let alice = credentials.merge(id_only);
// Merged values override existing values
let overridden = alice.merge(User::empty().id(21));
assert_eq!(overridden.name, "Alice");
assert_eq!(overridden.id, 21);
```

### Nested schemas

Schemas can contain other structural types:

```rust
#[structural]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[structural]
struct User<A: address::Fields> {
    id: u32,
    name: String,
    address: Address<A>,
}

// Require specific nested fields
fn ship<A, F>(user: User<A, F>)
where
    A: address::Fields<street = Present, city = Present>,
    F: user::Fields<id = Present, address = Present>
{
    println!("Shipping to {} at: {}, {}",
        user.id, user.address.street, user.address.city);
}
```

See [examples/](examples/) for more usage patterns, including a [REST API with SQLite](examples/todos-api/) demonstrating how one schema handles multiple endpoint types.

## License

MIT OR Apache-2.0
