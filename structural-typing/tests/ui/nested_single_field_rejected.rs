use structural_typing::structural;

#[structural]
struct Inner {
    data: String,
}

#[structural]
struct SingleField<I: inner::Fields> {
    #[nested(data)]
    inner: Inner<I>,
}

fn main() {}
