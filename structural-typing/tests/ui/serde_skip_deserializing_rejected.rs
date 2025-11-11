use serde::Deserialize;
use structural_typing::structural;

#[structural]
#[derive(Deserialize)]
struct User {
    name: String,
    #[serde(skip_deserializing)]
    email: String,
}

fn main() {}
