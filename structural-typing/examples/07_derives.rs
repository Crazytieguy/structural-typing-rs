//! Common trait derivation with structural types.

use structural_typing::structural;

#[structural]
#[derive(Clone, PartialEq, Eq, Debug)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    let user1 = User::empty().name("Alice".into()).id(123);

    // Clone works on partial structs
    let user2 = user1.clone();
    assert_eq!(user1, user2);

    // PartialEq compares field values
    let user3 = User::empty().name("Alice".into()).id(123);
    assert_eq!(user1, user3);

    let user4 = User::empty().name("Bob".into()).id(123);
    assert_ne!(user1, user4);

    // Different field configurations are different types
    let partial = User::empty().name("Alice".into());
    let complete = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into())
        .id(123);

    // This won't compile - different types:
    // assert_eq!(partial, complete);

    // But we can compare after projecting
    let complete_partial = complete.unset_email().unset_id();
    assert_eq!(partial, complete_partial);

    // Debug formatting works
    println!("{:?}", user1);
}
