//! Generic impls with conditional behavior.

use structural_typing::{access::Access, presence::Present, structural};

#[structural]
struct User {
    name: String,
    email: String,
}

// Requires name, behavior changes based on email presence
impl<F: user::Fields<name = Present>> User<F> {
    fn greet(&self) -> String {
        if let Some(email) = self.email.get() {
            format!("Hello, {} <{}>!", self.name, email)
        } else {
            format!("Hello, {}!", self.name)
        }
    }
}

fn main() {
    let user = user::empty().name("Alice".to_owned());
    assert_eq!(user.greet(), "Hello, Alice!");

    let user = user.email("alice@example.com".to_owned());
    assert_eq!(user.greet(), "Hello, Alice <alice@example.com>!");
}
