//! Serialization with different field states.

use serde::{Deserialize, Serialize};
use structural_typing::structural;

#[structural]
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    // Present fields serialize normally
    let user = User::empty().name("Alice".to_owned()).id(123);
    let json = serde_json::to_string(&user).unwrap();
    assert_eq!(json, r#"{"name":"Alice","id":123}"#);
    // email is Absent, so skipped

    // Optional(Some) serializes the value
    // Optional(None) serializes as null
    let user = User::empty()
        .name(Some("Bob".to_owned()))
        .email(None)
        .id(Some(456));
    let json = serde_json::to_string(&user).unwrap();
    assert_eq!(json, r#"{"name":"Bob","email":null,"id":456}"#);

    // AllAbsent serializes to empty object
    let empty = User::empty();
    let json = serde_json::to_string(&empty).unwrap();
    assert_eq!(json, "{}");

    // Deserialization works too
    let json = r#"{"name":"Charlie","email":"c@example.com","id":789}"#;
    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Charlie");
}
