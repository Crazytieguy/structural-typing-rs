//! Extracting field subsets with project() and select!().

use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    let full = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into())
        .id(123);

    // project() extracts a subset (compile-time checked)
    // select!(name, id) creates FieldSet<Present, Absent, Present>
    let subset = full.project::<user::select!(name, id)>();
    assert_eq!(subset.name, "Alice");
    assert_eq!(subset.id, 123);
    assert!(subset.get_email().is_none());

    // try_project() checks at runtime
    let maybe = User::empty()
        .maybe_name(Some("Bob".into()))
        .maybe_email(None); // None!

    // Fails because email is None but target needs Present
    let result = maybe.try_project::<user::select!(name, email)>();
    assert!(result.is_none());
}
