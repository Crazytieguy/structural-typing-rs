//! Extracting field subsets with extract() and select!().

use structural_typing::{select, structural};

#[structural]
#[derive(Clone, Debug, PartialEq)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    let user = user::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .id(123);

    let (credentials, remainder) = user.extract::<select!(user: name, email)>();
    assert_eq!(credentials.name, "Alice");
    assert_eq!(credentials.email, "alice@example.com");
    assert_eq!(remainder.id, 123);

    // Convert to Optional
    let partial = user::empty().name("Bob".to_owned());
    let (optional_creds, _) = partial.extract::<select!(user: name?, email?)>();
    assert_eq!(optional_creds.name, Some("Bob".to_owned()));
    assert_eq!(optional_creds.email, None);
}
