use crate::{
    generics_helpers::{IsNot, True},
    Access, Get, Property, P,
};

pub trait Set<T>: Access
where
    T: Property,
{
    type Output: Get<T>;
    fn _set(self, val: T::Type) -> Self::Output;
}

impl<U, T> Set<T> for U
where
    T: Property,
    Self: Get<T>,
{
    type Output = Self;
    fn _set(mut self, val: <T as Property>::Type) -> Self {
        *self.get_mut::<T>() = val;
        self
    }
}

impl<A, T> Set<T> for (P<A>,)
where
    A: Property,
    T: Property,
    IsNot<A, T>: True,
    (P<A>, P<T>): Get<T>,
{
    type Output = (P<A>, P<T>);
    fn _set(self, val: T::Type) -> (P<A>, P<T>) {
        (self.0, P(val))
    }
}

impl<A, B, T> Set<T> for (P<A>, P<B>)
where
    A: Property,
    B: Property,
    T: Property,
    IsNot<A, T>: True,
    IsNot<B, T>: True,
    (P<A>, P<B>, P<T>): Get<T>,
{
    type Output = (P<A>, P<B>, P<T>);
    fn _set(self, val: <T as Property>::Type) -> (P<A>, P<B>, P<T>) {
        (self.0, self.1, P(val))
    }
}

impl<A, B, C, T> Set<T> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
    IsNot<A, T>: True,
    IsNot<B, T>: True,
    IsNot<C, T>: True,
    T: Property,
    (P<A>, P<B>, P<C>, P<T>): Get<T>,
{
    type Output = (P<A>, P<B>, P<C>, P<T>);
    fn _set(self, val: <T as Property>::Type) -> (P<A>, P<B>, P<C>, P<T>) {
        (self.0, self.1, self.2, P(val))
    }
}
