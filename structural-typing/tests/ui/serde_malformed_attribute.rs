use structural_typing::structural;

#[structural]
#[derive(serde::Deserialize)]
struct Test {
    #[serde(invalid_attribute)]
    name: String,
}

fn main() {}
