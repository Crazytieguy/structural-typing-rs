pub trait Merge<T> {
    type Output;
    fn merge(self, other: T) -> Self::Output;
}
