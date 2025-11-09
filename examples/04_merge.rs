//! Combining partial structs with merge().

use structural_typing::structural;

#[structural]
struct User {
    name: String,
    id: u64,
}

fn main() {
    let with_name = User::empty().name("Bob".to_owned());
    let with_id = User::empty().id(123);

    let complete = with_name.merge(with_id);
    assert_eq!(complete.name, "Bob");
    assert_eq!(complete.id, 123);

    // Second wins
    let user1 = User::empty().name("Alice".to_owned()).id(111);
    let user2 = User::empty().name("Bob".to_owned()).id(222);
    let merged = user1.merge(user2);
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.id, 222);

    let optional_user = User::empty().name(Some("Charlie".to_owned()));
    let present_user = User::empty().name("David".to_owned());
    let merged = optional_user.merge(present_user);
    assert_eq!(merged.name, "David");
}
