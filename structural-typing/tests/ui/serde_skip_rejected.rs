use serde::Deserialize;
use structural_typing::structural;

#[structural]
#[derive(Deserialize)]
struct User {
    name: String,
    #[serde(skip)]
    email: String,
}

fn main() {}
