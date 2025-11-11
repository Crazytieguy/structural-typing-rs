#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};
use structural_typing::{select, structural};

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TestUser {
    name: String,
    email: String,
    id: u64,
}

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserWithRename {
    #[serde(rename = "user_name")]
    name: String,
    email: String,
    id: u64,
}

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserWithAlias {
    name: String,
    #[serde(alias = "mail")]
    email: String,
    id: u64,
}

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct UserWithRenameAll {
    user_name: String,
    email_address: String,
    user_id: u64,
}

#[test]
fn deserialize_full() {
    let json = r#"{"name":"Alice","email":"alice@test.com","id":123}"#;
    let user: TestUser = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@test.com");
    assert_eq!(user.id, 123);
}

#[test]
fn serialize_full() {
    let user = TestUser::empty()
        .name("Charlie".to_owned())
        .email("charlie@test.com".to_owned())
        .id(789);
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains(r#""name":"Charlie"#));
    assert!(json.contains(r#""email":"charlie@test.com"#));
}

#[test]
fn deserialize_partial() {
    let json_missing_email = r#"{"name":"Dave","id":999}"#;
    let user: TestUser<select!(test_user: name, id)> =
        serde_json::from_str(json_missing_email).unwrap();
    assert_eq!(user.name, "Dave");
    assert_eq!(user.id, 999);
}

#[test]
fn deserialize_missing_required_field_error() {
    let json_missing_name = r#"{"email":"test@test.com","id":1}"#;
    let result: Result<TestUser<select!(test_user: name, id)>, _> =
        serde_json::from_str(json_missing_name);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("missing field"));
}

#[test]
fn deserialize_with_rename_attr() {
    let json = r#"{"user_name":"Alice","email":"alice@test.com","id":123}"#;
    let user: UserWithRename = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@test.com");
    assert_eq!(user.id, 123);
}

#[test]
fn serialize_with_rename_attr() {
    let user = UserWithRename::empty()
        .name("Charlie".to_owned())
        .email("charlie@test.com".to_owned())
        .id(789);
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains(r#""user_name":"Charlie"#));
    assert!(!json.contains(r#""name":"#));
}

#[test]
fn deserialize_with_alias_primary_name() {
    let json = r#"{"name":"Dave","email":"dave@test.com","id":123}"#;
    let user: UserWithAlias = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Dave");
    assert_eq!(user.email, "dave@test.com");
}

#[test]
fn deserialize_with_alias_alternate_name() {
    let json = r#"{"name":"Eve","mail":"eve@test.com","id":456}"#;
    let user: UserWithAlias = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Eve");
    assert_eq!(user.email, "eve@test.com");
}

#[test]
fn deserialize_optional_field() {
    let json_with_email = r#"{"name":"Alice","email":"alice@test.com","id":123}"#;
    let user: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(json_with_email).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, Some("alice@test.com".to_owned()));
    assert_eq!(user.id, 123);

    let json_without_email = r#"{"name":"Bob","id":456}"#;
    let user2: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(json_without_email).unwrap();
    assert_eq!(user2.name, "Bob");
    assert_eq!(user2.email, None);
    assert_eq!(user2.id, 456);
}

#[test]
fn deserialize_mixed_presence_types() {
    let json = r#"{"name":"Alice","id":123}"#;
    let user: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, None);
    assert_eq!(user.id, 123);
}

#[test]
fn deserialize_extra_fields_with_absent() {
    let json = r#"{"name":"Alice","email":"alice@test.com","id":123,"extra":"ignored"}"#;
    let user: TestUser<select!(test_user: name, id)> = serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.id, 123);
}

#[test]
fn serialize_partial_omits_absent() {
    let user = TestUser::empty()
        .name("Alice".to_owned())
        .id(123);
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains(r#""name":"Alice"#));
    assert!(json.contains(r#""id":123"#));
    assert!(!json.contains("email"));
}

#[test]
fn deserialize_with_rename_all() {
    let json = r#"{"userName":"Alice","emailAddress":"alice@test.com","userId":123}"#;
    let user: UserWithRenameAll = serde_json::from_str(json).unwrap();
    assert_eq!(user.user_name, "Alice");
    assert_eq!(user.email_address, "alice@test.com");
    assert_eq!(user.user_id, 123);
}

#[test]
fn serialize_with_rename_all() {
    let user = UserWithRenameAll::empty()
        .user_name("Bob".to_owned())
        .email_address("bob@test.com".to_owned())
        .user_id(456);
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains(r#""userName":"Bob"#));
    assert!(json.contains(r#""emailAddress":"bob@test.com"#));
    assert!(json.contains(r#""userId":456"#));
    assert!(!json.contains("user_name"));
    assert!(!json.contains("email_address"));
    assert!(!json.contains("user_id"));
}

#[test]
fn serialize_with_alias() {
    let user = UserWithAlias::empty()
        .name("Charlie".to_owned())
        .email("charlie@test.com".to_owned())
        .id(789);
    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains(r#""name":"Charlie"#));
    assert!(json.contains(r#""email":"charlie@test.com"#));
    assert!(json.contains(r#""email":"#));
    assert!(!json.contains(r#""mail":"#));
}

#[test]
fn deserialize_null_for_present_field_errors() {
    let json = r#"{"name":null,"email":"alice@test.com","id":123}"#;
    let result: Result<TestUser, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn deserialize_null_for_optional_field() {
    let json = r#"{"name":"Alice","email":null,"id":123}"#;
    let user: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, None);
    assert_eq!(user.id, 123);
}

#[test]
fn deserialize_null_for_absent_field_ignored() {
    let json = r#"{"name":"Alice","email":null,"id":123}"#;
    let user: TestUser<select!(test_user: name, id)> =
        serde_json::from_str(json).unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.id, 123);
}

#[test]
fn roundtrip_full_struct() {
    let original = TestUser::empty()
        .name("Alice".to_owned())
        .email("alice@test.com".to_owned())
        .id(123);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: TestUser = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn roundtrip_partial_struct() {
    let original = TestUser::empty()
        .name("Bob".to_owned())
        .id(456);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: TestUser<select!(test_user: name, id)> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(original.name, deserialized.name);
    assert_eq!(original.id, deserialized.id);
}

#[test]
fn roundtrip_optional_fields() {
    let with_email = TestUser::empty()
        .name("Charlie".to_owned())
        .email(Some("charlie@test.com".to_owned()))
        .id(789);
    let json = serde_json::to_string(&with_email).unwrap();
    let deserialized: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(with_email.name, deserialized.name);
    assert_eq!(with_email.email, deserialized.email);
    assert_eq!(with_email.id, deserialized.id);

    let without_email = TestUser::empty()
        .name("Dave".to_owned())
        .email(None)
        .id(101);
    let json = serde_json::to_string(&without_email).unwrap();
    let deserialized: TestUser<select!(test_user: name, ?email, id)> =
        serde_json::from_str(&json).unwrap();
    assert_eq!(without_email.name, deserialized.name);
    assert_eq!(without_email.email, deserialized.email);
    assert_eq!(without_email.id, deserialized.id);
}

#[test]
fn error_message_includes_field_name() {
    let json_missing_name = r#"{"email":"test@test.com","id":1}"#;
    let result: Result<TestUser, _> = serde_json::from_str(json_missing_name);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("name") || err_msg.contains("missing"));
}

#[test]
fn error_message_for_multiple_missing_fields() {
    let json = r#"{"email":"test@test.com"}"#;
    let result: Result<TestUser, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Address {
    city: String,
    country: String,
}

#[structural]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UserWithAddress {
    name: String,
    address: Address,
}

#[test]
fn nested_structural_types() {
    let user = UserWithAddress::empty()
        .name("Alice".to_owned())
        .address(Address::empty()
            .city("Seattle".to_owned())
            .country("USA".to_owned()));

    let json = serde_json::to_string(&user).unwrap();
    let deserialized: UserWithAddress = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, "Alice");
    assert_eq!(deserialized.address.city, "Seattle");
    assert_eq!(deserialized.address.country, "USA");
}
