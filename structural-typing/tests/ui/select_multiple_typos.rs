use structural_typing::structural;

#[structural]
struct User {
    name: String,
    email: String,
    id: u64,
}

fn main() {
    // Multiple typos in one call
    type Invalid = user::select!(nane, emial, id);
}
