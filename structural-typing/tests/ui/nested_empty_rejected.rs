use structural_typing::structural;

#[structural]
struct Inner {
    data: String,
}

#[structural]
struct Outer<I: inner::Fields> {
    id: u64,
    #[nested(inner:)]
    inner: Inner<I>,
}

fn main() {}
