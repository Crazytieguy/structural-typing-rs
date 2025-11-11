# Structural Typing for Rust

Define a struct once and use it with different field combinations, tracked at compile time. Inspired by TypeScript, RDF, and [this talk](https://www.youtube.com/watch?v=YR5WdGrpoug).

**Status**: Experimental. API subject to change.

## Installation

```bash
cargo add structural-typing
```

If you use derives on your `#[structural]` structs, also add:
```bash
cargo add derive-where
```

## Example

```rust
use structural_typing::{structural, presence::Present, select};

#[structural]
struct User {
    id: u32,
    name: String,
    email: String,
}

type Create = select!(user: name, email);

fn create_user(data: User<Create>) -> User {
    data.id(generate_id())
}

fn update_user<F: user::Fields<id = Present>>(data: User<F>) {
    if let Some(name) = data.get_name() {
        update_in_db(data.id, name);
    }
}

fn main() {
    let new_user = User::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned());

    let user = create_user(new_user);

    let partial = User::empty().id(user.id).name(Some("Bob".to_owned()));
    update_user(partial);
}

fn generate_id() -> u32 { 42 }
fn update_in_db(id: u32, name: &String) { /* ... */ }
```

See [examples/](examples/) for comprehensive usage including merge, extract, serde integration, and more.

## Features

- Compile-time field requirements
- Builder API with type inference
- Merge and extract operations
- Serde support (via `serde` feature)
- Generic type parameters and nested structural types
- Zero runtime overhead

## Constraints

- Named structs only (not tuple structs or enums)
- At least one field required

## License

MIT OR Apache-2.0
