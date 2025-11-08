//! Extracting field subsets with split() and select!().

use structural_typing::structural;

#[structural]
#[derive(Clone, Debug, PartialEq)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    // split() returns (selected, remainder)
    let user = User::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .id(123);

    let (credentials, remainder) = user.split::<user::select!(name, email)>();
    assert_eq!(credentials.name, "Alice");
    assert_eq!(credentials.email, "alice@example.com");
    assert_eq!(remainder.id, 123);

    // Split-clone-merge pattern: duplicate a subset while preserving the whole
    let backup = credentials.clone();
    let user = credentials.merge(remainder);
    assert_eq!(user.name, backup.name);

    // try_split() succeeds when Optional fields have Some
    let complete = User::empty()
        .name(Some("Bob".to_owned()))
        .email(Some("bob@example.com".to_owned()))
        .id(456);

    let expected_credentials = User::empty()
        .name("Bob".to_owned())
        .email("bob@example.com".to_owned());
    let expected_remainder = User::empty().id(456);

    match complete.try_split::<user::select!(name, email)>() {
        Ok((credentials, remainder)) => {
            assert_eq!(credentials, expected_credentials);
            assert_eq!(remainder, expected_remainder);
        }
        Err(_) => panic!("Expected success"),
    }

    // try_split() fails if Optional field is None but target needs Present
    let partial = User::empty()
        .name(Some("Carol".to_owned()))
        .email(None)
        .id(789);

    let cloned = partial.clone();
    match partial.try_split::<user::select!(name, email)>() {
        Ok(_) => panic!("Expected error"),
        Err(original) => {
            assert_eq!(original, cloned); // Exact original returned
        }
    }
}
