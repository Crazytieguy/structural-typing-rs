use serde::Deserialize;
use structural_typing::structural;

#[structural]
#[derive(Deserialize)]
#[serde(default)]
struct User {
    name: String,
    email: String,
}

fn main() {}
