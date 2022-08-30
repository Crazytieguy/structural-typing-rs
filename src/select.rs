use crate::{Access, Has, Property, P};
use std::mem;

pub trait Select<T, U>: Access {
    fn _select(self) -> U;
}

impl<T: Property> Select<(T,), (P<T>,)> for (P<T>,) {
    fn _select(self) -> (P<T>,) {
        self
    }
}

impl<A, B, U> Select<(A, B), (P<A>, P<B>)> for U
where
    A: Property,
    B: Property,
    A::Type: Default,
    B::Type: Default,
    U: Has<A> + Has<B>,
{
    fn _select(mut self) -> (P<A>, P<B>) {
        let a = mem::take(self.get_mut::<A>());
        let b = mem::take(self.get_mut::<B>());
        (P(a), P(b))
    }
}

impl<A, B, C, U> Select<(A, B, C), (P<A>, P<B>, P<C>)> for U
where
    A: Property,
    B: Property,
    C: Property,
    A::Type: Default,
    B::Type: Default,
    C::Type: Default,
    U: Has<A> + Has<B> + Has<C>,
{
    fn _select(mut self) -> (P<A>, P<B>, P<C>) {
        let a = mem::take(self.get_mut::<A>());
        let b = mem::take(self.get_mut::<B>());
        let c = mem::take(self.get_mut::<C>());
        (P(a), P(b), P(c))
    }
}

impl<A, B, C, D, U> Select<(A, B, C, D), (P<A>, P<B>, P<C>, P<D>)> for U
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
    A::Type: Default,
    B::Type: Default,
    C::Type: Default,
    D::Type: Default,
    U: Has<A> + Has<B> + Has<C> + Has<D>,
{
    fn _select(mut self) -> (P<A>, P<B>, P<C>, P<D>) {
        let a = mem::take(self.get_mut::<A>());
        let b = mem::take(self.get_mut::<B>());
        let c = mem::take(self.get_mut::<C>());
        let d = mem::take(self.get_mut::<D>());
        (P(a), P(b), P(c), P(d))
    }
}
