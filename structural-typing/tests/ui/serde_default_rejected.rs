use serde::Deserialize;
use structural_typing::structural;

#[structural]
#[derive(Deserialize)]
struct User {
    name: String,
    #[serde(default)]
    email: String,
}

fn main() {}
