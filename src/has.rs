use crate::{Access, Property, P};

use std::marker::PhantomData;

auto trait True {}
struct IsNot<A, B>(PhantomData<A>, PhantomData<B>);
impl<T> !True for IsNot<T, T> {}

pub trait Has<T: Property>: Access {
    fn _get(&self) -> &T::Type;
    fn _get_mut(&mut self) -> &mut T::Type;
}

impl<T> Has<T> for (P<T>,)
where
    T: Property,
{
    fn _get(&self) -> &T::Type {
        &self.0 .0
    }
    fn _get_mut(&mut self) -> &mut T::Type {
        &mut self.0 .0
    }
}

impl<A, B> Has<A> for (P<A>, P<B>)
where
    A: Property,
    B: Property,
    IsNot<A, B>: True,
{
    fn _get(&self) -> &A::Type {
        &self.0 .0
    }
    fn _get_mut(&mut self) -> &mut A::Type {
        &mut self.0 .0
    }
}

impl<A, B> Has<B> for (P<A>, P<B>)
where
    A: Property,
    B: Property,
    IsNot<A, B>: True,
{
    fn _get(&self) -> &B::Type {
        &self.1 .0
    }
    fn _get_mut(&mut self) -> &mut B::Type {
        &mut self.1 .0
    }
}

impl<A, B, C> Has<A> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
    IsNot<A, B>: True,
    IsNot<A, C>: True,
{
    fn _get(&self) -> &A::Type {
        &self.0 .0
    }
    fn _get_mut(&mut self) -> &mut A::Type {
        &mut self.0 .0
    }
}

impl<A, B, C> Has<B> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
    IsNot<B, A>: True,
    IsNot<B, C>: True,
{
    fn _get(&self) -> &B::Type {
        &self.1 .0
    }
    fn _get_mut(&mut self) -> &mut B::Type {
        &mut self.1 .0
    }
}

impl<A, B, C> Has<C> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
    IsNot<C, A>: True,
    IsNot<C, B>: True,
{
    fn _get(&self) -> &C::Type {
        &self.2 .0
    }
    fn _get_mut(&mut self) -> &mut C::Type {
        &mut self.2 .0
    }
}

impl<A, B, C, D> Has<A> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
    IsNot<A, B>: True,
    IsNot<A, C>: True,
    IsNot<A, D>: True,
{
    fn _get(&self) -> &A::Type {
        &self.0 .0
    }
    fn _get_mut(&mut self) -> &mut A::Type {
        &mut self.0 .0
    }
}

impl<A, B, C, D> Has<B> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
    IsNot<B, A>: True,
    IsNot<B, C>: True,
    IsNot<B, D>: True,
{
    fn _get(&self) -> &B::Type {
        &self.1 .0
    }
    fn _get_mut(&mut self) -> &mut B::Type {
        &mut self.1 .0
    }
}

impl<A, B, C, D> Has<C> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
    IsNot<C, A>: True,
    IsNot<C, B>: True,
    IsNot<C, D>: True,
{
    fn _get(&self) -> &C::Type {
        &self.2 .0
    }
    fn _get_mut(&mut self) -> &mut C::Type {
        &mut self.2 .0
    }
}

impl<A, B, C, D> Has<D> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
    IsNot<D, A>: True,
    IsNot<D, B>: True,
    IsNot<D, C>: True,
{
    fn _get(&self) -> &D::Type {
        &self.3 .0
    }
    fn _get_mut(&mut self) -> &mut D::Type {
        &mut self.3 .0
    }
}
