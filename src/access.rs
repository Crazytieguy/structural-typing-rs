use crate::{into_values::IntoValues, select::Select, set::Set, Get, Property};

pub trait Access: Sized {
    fn get<'a, T>(&'a self) -> &'a T::Type
    where
        T: Property + 'a,
        Self: Get<T>,
    {
        Get::<T>::_get(self)
    }

    fn get_mut<'a, T>(&'a mut self) -> &'a mut T::Type
    where
        T: Property + 'a,
        Self: Get<T>,
    {
        Get::<T>::_get_mut(self)
    }

    fn select<T>(self) -> <Self as Select<T>>::Output
    where
        Self: Select<T>,
    {
        Select::<T>::_select(self)
    }

    fn into_values<T>(self) -> T
    where
        Self: IntoValues<T>,
    {
        IntoValues::<T>::_into_values(self)
    }

    fn set<T>(self, val: T::Type) -> <Self as Set<T>>::Output
    where
        T: Property,
        Self: Set<T>,
    {
        Set::<T>::_set(self, val)
    }
}

impl<T> Access for (T,) {}
impl<A, B> Access for (A, B) {}
impl<A, B, C> Access for (A, B, C) {}
impl<A, B, C, D> Access for (A, B, C, D) {}
