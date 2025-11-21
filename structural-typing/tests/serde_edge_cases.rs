#![cfg(feature = "serde")]

use serde::Deserialize;
use structural_typing::{select, structural};

#[structural]
#[derive(Deserialize, Debug, PartialEq)]
struct DeserializeOnly {
    name: String,
    value: u32,
}

#[test]
fn deserialize_only_struct() {
    let json = r#"{"name":"test","value":42}"#;
    let result: DeserializeOnly<deserialize_only::with::all> = serde_json::from_str(json).unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.value, 42);
}

#[test]
fn deserialize_only_partial() {
    let json = r#"{"name":"test","value":42}"#;
    let result: DeserializeOnly<select!(deserialize_only: name)> =
        serde_json::from_str(json).unwrap();
    assert_eq!(result.name, "test");
}
