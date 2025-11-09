use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn takes_full(_user: User) {}

fn main() {
    let user_empty = User::empty();
    takes_full(user_empty);
}
