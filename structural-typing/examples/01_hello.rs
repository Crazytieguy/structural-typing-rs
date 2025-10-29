//! Basic incremental building with type-safe field requirements.

use structural_typing::{presence::Present, structural};

#[structural]
struct User {
    name: String,
    email: String,
}

// Methods can require specific fields be Present
impl<F: user::Fields<name = Present>> User<F> {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    // Build incrementally - type tracks which fields are set
    let user = User::empty().name("Alice".into());

    // ✓ Compiles - name is Present
    assert_eq!(user.greet(), "Hello, Alice!");

    // Continue building
    let user = user.email("alice@example.com".into());
    assert_eq!(user.email, "alice@example.com");
}
