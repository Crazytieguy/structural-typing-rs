#![allow(clippy::trait_duplication_in_bounds)]
#![allow(clippy::mismatching_type_param_order)]

use std::{marker::PhantomData, mem};

pub trait Property {
    type Type;
}

pub struct P<T: Property>(pub T::Type);

pub trait Access: Sized {
    fn get<'a, T>(&'a self) -> &'a T::Type
    where
        T: Property + 'a,
        Self: Has<T>,
    {
        Has::<T>::_get(self)
    }

    fn get_mut<'a, T>(&'a mut self) -> &'a mut T::Type
    where
        T: Property + 'a,
        Self: Has<T>,
    {
        Has::<T>::_get_mut(self)
    }

    fn select<T, U>(self) -> U
    where
        Self: Select<T, U>,
    {
        Select::<T, U>::_select(self)
    }

    fn into_values<T>(self) -> T
    where
        Self: IntoValues<T>,
    {
        IntoValues::<T>::_into_values(self)
    }
}

pub trait Has<T: Property>: Access {
    fn _get(&self) -> &T::Type;
    fn _get_mut(&mut self) -> &mut T::Type;
}

pub trait Select<T, U>: Access {
    fn _select(self) -> U;
}

pub trait IntoValues<T>: Access {
    fn _into_values(self) -> T;
}

auto trait True {}

struct IsNot<A, B>(PhantomData<A>, PhantomData<B>);
impl<T> !True for IsNot<T, T> {}

impl<T> Access for (T,) {}
impl<A, B> Access for (A, B) {}
impl<A, B, C> Access for (A, B, C) {}
impl<A, B, C, D> Access for (A, B, C, D) {}

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

impl<A> IntoValues<(A::Type,)> for (P<A>,)
where
    A: Property,
{
    fn _into_values(self) -> (A::Type,) {
        (self.0 .0,)
    }
}

impl<A, B> IntoValues<(A::Type, B::Type)> for (P<A>, P<B>)
where
    A: Property,
    B: Property,
{
    fn _into_values(self) -> (A::Type, B::Type) {
        (self.0 .0, self.1 .0)
    }
}

impl<A, B, C> IntoValues<(A::Type, B::Type, C::Type)> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
{
    fn _into_values(self) -> (A::Type, B::Type, C::Type) {
        (self.0 .0, self.1 .0, self.2 .0)
    }
}

impl<A, B, C, D> IntoValues<(A::Type, B::Type, C::Type, D::Type)> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
{
    fn _into_values(self) -> (A::Type, B::Type, C::Type, D::Type) {
        (self.0 .0, self.1 .0, self.2 .0, self.3 .0)
    }
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
