//! Basic incremental building.

use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    // Build incrementally - type tracks which fields are set
    let user = User::empty().name("Alice".to_owned());
    assert_eq!(user.name, "Alice");

    // Continue building
    let user = user.email("alice@example.com".to_owned());
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}
