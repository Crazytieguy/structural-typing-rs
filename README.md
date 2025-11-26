# Structural Typing for Rust

**Status**: Experimental. API subject to change.

## Overview

Structural typing separates schemas (what fields can exist) from selections (what fields must be present). Optionality is context-dependent—a name is always a string, but different contexts may or may not require it. Instead of creating separate types for each context (`CreateUser`, `UpdateUser`, `DBUser`...) or using `Option<T>` everywhere, define your schema once and use generic trait bounds for context-specific requirements—all verified at compile time.

This design is inspired by Rich Hickey's [Maybe Not](https://www.youtube.com/watch?v=YR5WdGrpoug) talk, which articulates the principle of separating attribute definitions from their use in aggregates, along with ideas from TypeScript's structural types and RDF's independent attributes.

## Installation

```toml
[dependencies]
structural-typing = "0.1.9"
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
let bob = user::empty().id(123);
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
let overridden = alice.merge(user::empty().id(21));
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
fn ship<F, A>(user: User<F, A>)
where
    A: address::Fields<street = Present, city = Present>,
    F: user::Fields<id = Present, address = Present>
{
    println!("Shipping to {} at: {}, {}",
        user.id, user.address.street, user.address.city);
}
```

#### Nested setters

Use `#[nested(...)]` to generate convenient setters for nested struct fields:

```rust
#[structural]
struct User<A: address::Fields = select!(address: all-)> {
    id: u32,
    name: String,
    #[nested(address: street, city, zip)]
    address: Address<A>,
}

// Setters are generated with field_nested_field naming
let user = user::empty()
    .id(1)
    .name("Alice".to_owned())
    .address(address::empty())
    .address_city("Seattle".to_owned())
    .address_street("Main St".to_owned());

assert_eq!(user.address.city, "Seattle");
assert_eq!(user.address.street, "Main St");
```

Nested setters work by extracting the nested field, updating it, and merging back—preserving all other fields' presence states. This makes them compatible with `extract()` and other structural operations.

**Limitations**:
- `#[nested]` requires the parent struct to have at least 2 fields because the implementation uses the spread operator, which needs other fields to preserve. Single-field parent structs will produce a compile error.
- Merge operations between instances with different nested field presence states are not supported. The nested struct's generic parameter must match for merge to work.

See [examples/](examples/) for more usage patterns, including a [REST API with SQLite](examples/todos-api/) demonstrating how one schema handles multiple endpoint types.

## License

MIT OR Apache-2.0
