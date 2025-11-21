use structural_typing::structural;

#[structural]
struct User {
    name: String,
}

fn takes_full(_user: User<user::with::all>) {}

fn main() {
    let user_empty = user::empty();
    takes_full(user_empty);
}
