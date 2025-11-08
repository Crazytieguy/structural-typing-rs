//! Type algebra with select! macro and generic methods.

use structural_typing::{select, structural};

#[structural]
struct User {
    name: String,
    email: String,
    id: u64,
}

// Generic method works with ANY field configuration
impl<F: user::Fields> User<F> {
    fn describe(&self) -> String {
        let mut parts = vec![];
        if let Some(name) = self.get_name() {
            parts.push(format!("name: {}", name));
        }
        if let Some(email) = self.get_email() {
            parts.push(format!("email: {}", email));
        }
        if let Some(id) = self.get_id() {
            parts.push(format!("id: {}", id));
        }

        if parts.is_empty() {
            "User (no fields)".into()
        } else {
            format!("User {{ {} }}", parts.join(", "))
        }
    }
}

fn main() {
    // select! composes type aliases to create custom field sets
    // Expands to: user::with::name::Present<user::with::email::Present>
    type NameEmail = select!(user: name, email);
    let user1: User<NameEmail> = User::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned());
    assert_eq!(
        user1.describe(),
        "User { name: Alice, email: alice@example.com }"
    );

    // Single field: expands to user::with::name::Present
    type NameOnly = select!(user: name);
    let user2: User<NameOnly> = User::empty().name("Bob".to_owned());
    assert_eq!(user2.describe(), "User { name: Bob }");

    // Generic method works with any configuration
    let full = User::empty()
        .name("Charlie".to_owned())
        .email("c@ex.com".to_owned())
        .id(123);
    assert_eq!(
        full.describe(),
        "User { name: Charlie, email: c@ex.com, id: 123 }"
    );

    let empty = User::empty();
    assert_eq!(empty.describe(), "User (no fields)");
}
