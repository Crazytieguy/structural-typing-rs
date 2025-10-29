//! Type algebra with modify!() and generic methods.

use structural_typing::structural;

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
    // modify!() transforms existing FieldSets
    // +field = add as Present, -field = remove, ?field = make Optional

    type NameEmail = user::modify!(user::AllAbsent, +name, +email);
    let user1: User<NameEmail> = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into());
    assert_eq!(user1.describe(), "User { name: Alice, email: alice@example.com }");

    type NameOnly = user::modify!(user::AllPresent, -email, -id);
    let user2: User<NameOnly> = User::empty().name("Bob".into());
    assert_eq!(user2.describe(), "User { name: Bob }");

    // Generic method works with any configuration
    let full = User::empty().name("Charlie".into()).email("c@ex.com".into()).id(123);
    assert_eq!(full.describe(), "User { name: Charlie, email: c@ex.com, id: 123 }");

    let empty: User<user::AllAbsent> = User::empty();
    assert_eq!(empty.describe(), "User (no fields)");
}
