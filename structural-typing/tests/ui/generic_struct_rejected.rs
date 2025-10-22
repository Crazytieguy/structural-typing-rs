use structural_typing::structural;

#[structural]
struct Generic<T> {
    value: T,
}

fn main() {}
