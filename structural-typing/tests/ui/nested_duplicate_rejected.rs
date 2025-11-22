use structural_typing::structural;

#[structural]
struct Inner {
    data: String,
    label: String,
}

#[structural]
struct Outer<I: inner::Fields> {
    id: u64,
    #[nested(inner: data, data)]
    inner: Inner<I>,
}

fn main() {}
