//! Fallible extraction with try_extract() for Optional→Present conversion.

use structural_typing::{select, structural};

#[structural]
#[derive(Clone, Debug, PartialEq)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    // Success case
    let complete = User::empty()
        .name(Some("Bob".to_owned()))
        .email(Some("bob@example.com".to_owned()))
        .id(456);

    let expected_credentials = User::empty()
        .name("Bob".to_owned())
        .email("bob@example.com".to_owned());
    let expected_remainder = User::empty().id(456);

    match complete.try_extract::<select!(user: name, email)>() {
        Ok((credentials, remainder)) => {
            assert_eq!(credentials, expected_credentials);
            assert_eq!(remainder, expected_remainder);
        }
        Err(_) => panic!("Expected success"),
    }

    // Failure case
    let partial = User::empty()
        .name(Some("Carol".to_owned()))
        .email(None)
        .id(789);

    let cloned = partial.clone();
    match partial.try_extract::<select!(user: name, email)>() {
        Ok(_) => panic!("Expected error"),
        Err(original) => {
            assert_eq!(original, cloned);
        }
    }
}
