use serde::Deserialize;
use structural_typing::structural;

#[structural]
#[derive(Deserialize)]
#[serde(transparent)]
struct UserId {
    id: u64,
}

fn main() {}
