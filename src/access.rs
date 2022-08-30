use crate::{into_values::IntoValues, select::Select, Has, Property};

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

impl<T> Access for (T,) {}
impl<A, B> Access for (A, B) {}
impl<A, B, C> Access for (A, B, C) {}
impl<A, B, C, D> Access for (A, B, C, D) {}
