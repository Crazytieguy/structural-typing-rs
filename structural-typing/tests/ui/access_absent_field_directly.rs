use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn main() {
    let user = User::empty();
    let _name_str: String = user.name;
}
