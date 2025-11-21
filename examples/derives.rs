//! Common trait derivation with structural types.

use structural_typing::{select, structural};

#[structural]
#[derive(Clone, PartialEq, Eq, Debug)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    let user1 = user::empty().name("Alice".to_owned()).id(123);

    let user2 = user1.clone();
    assert_eq!(user1, user2);

    let user3 = user::empty().name("Alice".to_owned()).id(123);
    assert_eq!(user1, user3);

    let user4 = user::empty().name("Bob".to_owned()).id(123);
    assert_ne!(user1, user4);

    // Different field configurations are different types
    let partial = user::empty().name("Alice".to_owned());
    let complete = user::empty()
        .name("Alice".to_owned())
        .email("alice@example.com".to_owned())
        .id(123);

    // Won't compile - different types:
    // assert_eq!(partial, complete);

    // Compare after extract
    let (complete_partial, _) = complete.extract::<select!(user: name)>();
    assert_eq!(partial, complete_partial);

    println!("{:?}", user1);
}
