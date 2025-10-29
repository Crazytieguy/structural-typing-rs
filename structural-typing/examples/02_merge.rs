//! Combining partial structs with merge().

use structural_typing::structural;

#[structural]
struct User {
    name: String,
    id: u64,
}

fn main() {
    // Build two partial users
    let with_name = User::empty().name("Bob".into());
    let with_id = User::empty().id(123);

    // Merge combines them
    let complete = with_name.merge(with_id);
    assert_eq!(complete.name, "Bob");
    assert_eq!(complete.id, 123);

    // Conflict resolution: second argument wins
    let user1 = User::empty().name("Alice".into()).id(111);
    let user2 = User::empty().name("Bob".into()).id(222);
    let merged = user1.merge(user2);
    assert_eq!(merged.name, "Bob"); // user2's value
    assert_eq!(merged.id, 222);
}
