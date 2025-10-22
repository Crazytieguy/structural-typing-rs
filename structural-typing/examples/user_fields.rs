use structural_typing::{
    presence::{Absent, Optional, Present},
    structural,
};

fn main() {
    // Basic usage with Present fields
    let user = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into());
    assert_eq!(user.greet(), "Hello, Alice!");
    assert_eq!(user.email_subject(), "Welcome, Alice! <alice@example.com>");
    assert_eq!(user.email, "alice@example.com");

    let user_with_id = user.id(12345);
    assert_eq!(user_with_id.id, 12345);

    // Merge functionality - combines fields from two partial users
    let partial_user = User::empty().name("Bob".into());
    let id_user = User::empty().id(67890);
    let merged = partial_user.merge(id_user);
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.id, 67890);

    // Merge conflict resolution: second argument (other) wins
    let user1 = User::empty().name("Alice".into()).id(111);
    let user2 = User::empty().name("Bob".into()).id(222);
    let merged_conflict = user1.merge(user2);
    assert_eq!(merged_conflict.name, "Bob"); // user2 wins
    assert_eq!(merged_conflict.id, 222); // user2 wins

    // Optional fields
    let optional_user = User::empty().maybe_name(Some("Charlie".into()));
    assert_eq!(optional_user.name, Some("Charlie".to_string()));

    // Absent fields - using generated accessor methods
    let absent_user = User::empty().unset_name();
    assert!(absent_user.get_name().is_none());

    // Clone and Debug work with all FieldSet variations
    let full_user = User::empty()
        .name("David".into())
        .email("david@example.com".into())
        .id(42);
    let cloned = full_user.clone();
    assert_eq!(cloned.name, "David");
    assert_eq!(cloned.email, "david@example.com");
    assert_eq!(cloned.id, 42);

    let partial = User::empty().name("Partial".into());
    let partial_clone = partial.clone();
    assert_eq!(partial_clone.name, "Partial");

    let optional = User::empty()
        .maybe_name(Some("Maybe".into()))
        .maybe_email(None)
        .maybe_id(Some(999));
    let optional_clone = optional.clone();
    assert_eq!(optional_clone.name, Some("Maybe".to_string()));
    assert_eq!(optional_clone.email, None);
    assert_eq!(optional_clone.id, Some(999));

    let empty = User::empty();
    let empty_clone = empty.clone();
    assert!(empty_clone.get_name().is_none());

    // Default works with all FieldSet variations
    let default_absent: User<user::AllAbsent> = Default::default();
    assert!(default_absent.get_name().is_none());

    let default_present: User<user::AllPresent> = Default::default();
    assert_eq!(default_present.name, "");
    assert_eq!(default_present.email, "");
    assert_eq!(default_present.id, 0);

    // Projection
    // user::select!(name, id) → FieldSet<Present, Absent, Present>
    let projected = full_user.project::<user::FieldSet<Present, Absent, Present>>();
    assert_eq!(projected.name, "David");
    assert_eq!(projected.id, 42);

    let maybe_user = User::empty()
        .maybe_name(Some("Eve".into()))
        .maybe_email(None);
    // user::select!(name, email) → FieldSet<Present, Present, Absent>
    let try_projected = maybe_user.try_project::<user::FieldSet<Present, Present, Absent>>();
    assert!(try_projected.is_none(), "Should fail due to missing email");

    // Serde: works with all FieldSet variations
    let full_user = User::empty()
        .name("Frank".into())
        .email("frank@example.com".into())
        .id(123);
    let json = serde_json::to_string(&full_user).unwrap();
    assert!(json.contains(r#""name":"Frank"#));
    assert!(json.contains(r#""email":"frank@example.com"#));
    assert!(json.contains(r#""id":123"#));

    let deserialized: User<user::AllPresent> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Frank");
    assert_eq!(deserialized.email, "frank@example.com");
    assert_eq!(deserialized.id, 123);

    let partial_user = User::empty().name("Grace".into()).id(456);
    let partial_json = serde_json::to_string(&partial_user).unwrap();
    assert!(partial_json.contains(r#""name":"Grace"#));
    assert!(partial_json.contains(r#""id":456"#));
    assert!(
        !partial_json.contains("email"),
        "Absent field should be skipped!"
    );

    let optional_user = User::empty()
        .maybe_name(Some("Henry".into()))
        .maybe_email(None)
        .maybe_id(Some(789));
    let optional_json = serde_json::to_string(&optional_user).unwrap();
    assert!(optional_json.contains(r#""name":"Henry"#));
    assert!(optional_json.contains(r#""email":null"#)); // Optional None -> null
    assert!(optional_json.contains(r#""id":789"#));

    // Test various combinations of Present/Optional/Absent
    // user::select!(name, ?email) → FieldSet<Present, Optional, Absent>
    let mixed1: User<user::FieldSet<Present, Optional, Absent>> = User::empty()
        .name("Alice".into())
        .maybe_email(Some("alice@test.com".into()));
    let mixed1_json = serde_json::to_string(&mixed1).unwrap();
    assert_eq!(mixed1_json, r#"{"name":"Alice","email":"alice@test.com"}"#);

    // user::select!(email, ?id) → FieldSet<Absent, Present, Optional>
    let mixed2: User<user::FieldSet<Absent, Present, Optional>> =
        User::empty().email("bob@test.com".into()).maybe_id(None);
    let mixed2_json = serde_json::to_string(&mixed2).unwrap();
    assert_eq!(mixed2_json, r#"{"email":"bob@test.com","id":null}"#);

    let all_absent: User<user::AllAbsent> = User::empty();
    let all_absent_json = serde_json::to_string(&all_absent).unwrap();
    assert_eq!(all_absent_json, "{}");

    // user::modify! examples - modify existing FieldSets
    // user::modify!(AllAbsent, +name, +email) → FieldSet<Present, Present, Absent>
    type NameEmail = user::FieldSet<Present, Present, Absent>;
    let _partial: User<NameEmail> = User::empty()
        .name("test".into())
        .email("test@example.com".into());

    // user::modify!(AllPresent, -email, ?id) → FieldSet<Present, Absent, Optional>
    type NameMaybeId = user::FieldSet<Present, Absent, Optional>;
    let _mixed: User<NameMaybeId> = User::empty().name("test".into()).maybe_id(Some(42));

    // Generic methods using getters - same impl works with any FieldSet
    let full_user = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into())
        .id(123);
    assert_eq!(
        full_user.describe(),
        "User { name: Alice, email: alice@example.com, id: 123 }"
    );

    let partial_user = User::empty().name("Bob".into()).id(456);
    assert_eq!(partial_user.describe(), "User { name: Bob, id: 456 }");

    let name_only = User::empty().name("Charlie".into());
    assert_eq!(name_only.describe(), "User { name: Charlie }");

    let optional_user = User::empty()
        .maybe_name(Some("Diana".into()))
        .maybe_email(None)
        .maybe_id(Some(789));
    assert_eq!(optional_user.describe(), "User { name: Diana, id: 789 }");

    let empty_user: User<user::AllAbsent> = User::empty();
    assert_eq!(empty_user.describe(), "User (no fields set)");

    println!("✓ All assertions passed!");
}

// User writes this:
#[structural]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct User {
    pub name: String,
    pub email: String,
    pub id: u64,
}

// User-defined bounded implementations
impl<F: user::Fields<name = Present>> User<F> {
    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl<F: user::Fields<name = Present, email = Present>> User<F> {
    pub fn email_subject(&self) -> String {
        format!("Welcome, {}! <{}>", self.name, self.email)
    }
}

impl<F: user::Fields> User<F> {
    pub fn describe(&self) -> String {
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
            "User (no fields set)".to_string()
        } else {
            format!("User {{ {} }}", parts.join(", "))
        }
    }
}
