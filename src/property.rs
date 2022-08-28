#![allow(clippy::trait_duplication_in_bounds)]
#![allow(clippy::mismatching_type_param_order)]

use std::marker::PhantomData;

auto trait True {}

struct IsNot<A, B>(PhantomData<A>, PhantomData<B>);
impl<T> !True for IsNot<T, T> {}

pub trait Property {
    type Item;
    fn get(&self) -> &Self::Item;
}

pub trait Access {
    fn get<'a, T>(&'a self) -> &'a T::Item
    where
        T: Property + 'a,
        Self: Has<T>,
    {
        Has::<T>::_get(self).get()
    }
}

impl<T> Access for (T,) {}
impl<A, B> Access for (A, B) {}
impl<A, B, C> Access for (A, B, C) {}
impl<A, B, C, D> Access for (A, B, C, D) {}

pub trait Has<T>: Access {
    fn _get(&self) -> &T;
}

impl<T> Has<T> for (T,) {
    fn _get(&self) -> &T {
        &self.0
    }
}

impl<A, B> Has<A> for (A, B)
where
    IsNot<A, B>: True,
{
    fn _get(&self) -> &A {
        &self.0
    }
}

impl<A, B> Has<B> for (A, B)
where
    IsNot<A, B>: True,
{
    fn _get(&self) -> &B {
        &self.1
    }
}

impl<A, B, C> Has<A> for (A, B, C)
where
    IsNot<A, B>: True,
    IsNot<A, C>: True,
{
    fn _get(&self) -> &A {
        &self.0
    }
}

impl<A, B, C> Has<B> for (A, B, C)
where
    IsNot<B, A>: True,
    IsNot<B, C>: True,
{
    fn _get(&self) -> &B {
        &self.1
    }
}

impl<A, B, C> Has<C> for (A, B, C)
where
    IsNot<C, A>: True,
    IsNot<C, B>: True,
{
    fn _get(&self) -> &C {
        &self.2
    }
}

impl<A, B, C, D> Has<A> for (A, B, C, D)
where
    IsNot<A, B>: True,
    IsNot<A, C>: True,
    IsNot<A, D>: True,
{
    fn _get(&self) -> &A {
        &self.0
    }
}

impl<A, B, C, D> Has<B> for (A, B, C, D)
where
    IsNot<B, A>: True,
    IsNot<B, C>: True,
    IsNot<B, D>: True,
{
    fn _get(&self) -> &B {
        &self.1
    }
}

impl<A, B, C, D> Has<C> for (A, B, C, D)
where
    IsNot<C, A>: True,
    IsNot<C, B>: True,
    IsNot<C, D>: True,
{
    fn _get(&self) -> &C {
        &self.2
    }
}

impl<A, B, C, D> Has<D> for (A, B, C, D)
where
    IsNot<D, A>: True,
    IsNot<D, B>: True,
    IsNot<D, C>: True,
{
    fn _get(&self) -> &D {
        &self.3
    }
}