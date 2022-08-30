use std::marker::PhantomData;

pub(crate) auto trait True {}
pub(crate) struct IsNot<A, B>(PhantomData<(A, B)>);
impl<T> !True for IsNot<T, T> {}
