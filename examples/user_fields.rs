use structural_typing::{
    access::{Access, is_absent},
    presence::{Absent, Optional, Present, Presence, Project, TryProject},
};
use std::marker::PhantomData;
use derive_where::derive_where;

use crate::user::{AllAbsent, AllPresent};
// Note: Serialize/Deserialize traits are used via derive_where, no need to import

fn main() {
    // Basic usage with Present fields
    let user = User::empty()
        .name("Alice".into())
        .email("alice@example.com".into());
    assert_eq!(user.greet(), "Hello, Alice!");
    assert_eq!(user.email, "alice@example.com");

    let user_with_id = user.id(12345);
    assert_eq!(user_with_id.get_id(), 12345);

    // Merge functionality
    let partial_user = User::empty().name("Bob".into());
    let id_user = User::empty().id(67890);
    let merged = partial_user.merge(id_user);
    assert_eq!(merged.name, "Bob");
    assert_eq!(merged.get_id(), 67890);

    // Optional fields
    let optional_user = User::empty().maybe_name(Some("Charlie".into()));
    assert_eq!(optional_user.name, Some("Charlie".to_string()));

    // Absent fields (Access trait)
    let absent_user = User::empty().unset_name();
    assert!(Access::<String>::get(&absent_user.name).is_none());

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
    assert!(Access::<String>::get(&empty_clone.name).is_none());

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

    let deserialized: User<AllPresent> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, "Frank");
    assert_eq!(deserialized.email, "frank@example.com");
    assert_eq!(deserialized.id, 123);

    let partial_user = User::empty()
        .name("Grace".into())
        .id(456);
    let partial_json = serde_json::to_string(&partial_user).unwrap();
    assert!(partial_json.contains(r#""name":"Grace"#));
    assert!(partial_json.contains(r#""id":456"#));
    assert!(!partial_json.contains("email"), "Absent field should be skipped!");

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
    let mixed2: User<user::FieldSet<Absent, Present, Optional>> = User::empty()
        .email("bob@test.com".into())
        .maybe_id(None);
    let mixed2_json = serde_json::to_string(&mixed2).unwrap();
    assert_eq!(mixed2_json, r#"{"email":"bob@test.com","id":null}"#);

    let all_absent: User<AllAbsent> = User::empty();
    let all_absent_json = serde_json::to_string(&all_absent).unwrap();
    assert_eq!(all_absent_json, "{}");

    // user::modify! examples - modify existing FieldSets
    // user::modify!(AllAbsent, +name, +email) → FieldSet<Present, Present, Absent>
    type NameEmail = user::FieldSet<Present, Present, Absent>;
    let _partial: User<NameEmail> = User::empty().name("test".into()).email("test@example.com".into());

    // user::modify!(AllPresent, -email, ?id) → FieldSet<Present, Absent, Optional>
    type NameMaybeId = user::FieldSet<Present, Absent, Optional>;
    let _mixed: User<NameMaybeId> = User::empty().name("test".into()).maybe_id(Some(42));

    println!("✓ All assertions passed!");
}

impl<F: user::Fields<name = Present>> User<F> {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

impl<F: user::Fields<id = Present>> User<F> {
    fn get_id(&self) -> u64 {
        self.id
    }
}

// ============================================================================
// Generated module (from #[structural] macro):
// ============================================================================
mod user {
    use super::*;

    mod sealed {
        pub trait Sealed {}
    }

    #[allow(non_camel_case_types)]
    pub trait Fields: sealed::Sealed {
        type name: Presence;
        type email: Presence;
        type id: Presence;
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug)]
    pub struct FieldSet<name: Presence, email: Presence, id: Presence>(
        PhantomData<(name, email, id)>,
    );

    #[allow(non_camel_case_types)]
    impl<name: Presence, email: Presence, id: Presence> sealed::Sealed for FieldSet<name, email, id> {}

    #[allow(non_camel_case_types)]
    impl<name: Presence, email: Presence, id: Presence> Fields for FieldSet<name, email, id> {
        type name = name;
        type email = email;
        type id = id;
    }

    pub type Merge<F1, F2> = FieldSet<
        <<F2 as Fields>::name as Presence>::Or<<F1 as Fields>::name>,
        <<F2 as Fields>::email as Presence>::Or<<F1 as Fields>::email>,
        <<F2 as Fields>::id as Presence>::Or<<F1 as Fields>::id>,
    >;

    // Type aliases for common FieldSet patterns
    pub type AllPresent = FieldSet<Present, Present, Present>;
    pub type AllOptional = FieldSet<Optional, Optional, Optional>;
    pub type AllAbsent = FieldSet<Absent, Absent, Absent>;
}

// ============================================================================
// What the user would write with the #[structural] macro:
// ============================================================================
// #[structural]
// #[derive(Clone, Debug, Serialize, Deserialize)]
// struct User {
//     name: String,
//     email: String,
//     id: u64,
// }
// ============================================================================

// What the #[structural] macro would generate:
#[derive_where(Clone, Debug, Serialize, Deserialize;
    <F::name as Presence>::Output<String>,
    <F::email as Presence>::Output<String>,
    <F::id as Presence>::Output<u64>
)]
struct User<F: user::Fields = user::FieldSet<Absent, Absent, Absent>> {
    #[serde(skip_serializing_if = "is_absent")]
    pub name: <F::name as Presence>::Output<String>,
    #[serde(skip_serializing_if = "is_absent")]
    pub email: <F::email as Presence>::Output<String>,
    #[serde(skip_serializing_if = "is_absent")]
    pub id: <F::id as Presence>::Output<u64>,
}

// ============================================================================
// Generated impl blocks (from #[structural] macro):
// ============================================================================

impl User {
    fn empty() -> Self {
        Self {
            name: PhantomData,
            email: PhantomData,
            id: PhantomData,
        }
    }
}

impl<F: user::Fields> User<F> {
    fn name(self, name: String) -> User<user::FieldSet<Present, F::email, F::id>> {
        User {
            name,
            email: self.email,
            id: self.id,
        }
    }

    fn maybe_name(self, name: Option<String>) -> User<user::FieldSet<Optional, F::email, F::id>> {
        User {
            name,
            email: self.email,
            id: self.id,
        }
    }

    fn unset_name(self) -> User<user::FieldSet<Absent, F::email, F::id>> {
        User {
            name: PhantomData,
            email: self.email,
            id: self.id,
        }
    }

    fn email(self, email: String) -> User<user::FieldSet<F::name, Present, F::id>> {
        User {
            name: self.name,
            email,
            id: self.id,
        }
    }

    fn maybe_email(self, email: Option<String>) -> User<user::FieldSet<F::name, Optional, F::id>> {
        User {
            name: self.name,
            email,
            id: self.id,
        }
    }

    fn unset_email(self) -> User<user::FieldSet<F::name, Absent, F::id>> {
        User {
            name: self.name,
            email: PhantomData,
            id: self.id,
        }
    }

    fn id(self, id: u64) -> User<user::FieldSet<F::name, F::email, Present>> {
        User {
            name: self.name,
            email: self.email,
            id,
        }
    }

    fn maybe_id(self, id: Option<u64>) -> User<user::FieldSet<F::name, F::email, Optional>> {
        User {
            name: self.name,
            email: self.email,
            id,
        }
    }

    fn unset_id(self) -> User<user::FieldSet<F::name, F::email, Absent>> {
        User {
            name: self.name,
            email: self.email,
            id: PhantomData,
        }
    }

    fn merge<F2: user::Fields>(self, other: User<F2>) -> User<user::Merge<F, F2>> {
        User {
            name: <F2::name as Presence>::or(other.name, self.name),
            email: <F2::email as Presence>::or(other.email, self.email),
            id: <F2::id as Presence>::or(other.id, self.id),
        }
    }

    fn project<F2: user::Fields>(self) -> User<F2>
    where
        F::name: Project<F2::name>,
        F::email: Project<F2::email>,
        F::id: Project<F2::id>,
    {
        User {
            name: <F::name as Project<F2::name>>::project(self.name),
            email: <F::email as Project<F2::email>>::project(self.email),
            id: <F::id as Project<F2::id>>::project(self.id),
        }
    }

    fn try_project<F2: user::Fields>(self) -> Option<User<F2>>
    where
        F::name: TryProject<F2::name>,
        F::email: TryProject<F2::email>,
        F::id: TryProject<F2::id>,
    {
        Some(User {
            name: <F::name as TryProject<F2::name>>::try_project(self.name)?,
            email: <F::email as TryProject<F2::email>>::try_project(self.email)?,
            id: <F::id as TryProject<F2::id>>::try_project(self.id)?,
        })
    }
}
