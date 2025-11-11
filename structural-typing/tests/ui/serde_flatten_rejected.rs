use serde::Deserialize;
use structural_typing::structural;

#[derive(Deserialize)]
struct Address {
    city: String,
}

#[structural]
#[derive(Deserialize)]
struct User {
    name: String,
    #[serde(flatten)]
    address: Address,
}

fn main() {}
