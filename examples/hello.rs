//! Basic incremental building.

use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
}

fn main() {
    // Build incrementally
    let user = user::empty().name("Alice".to_owned());
    assert_eq!(user.name, "Alice");

    let user = user.email("alice@example.com".to_owned());
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
}
