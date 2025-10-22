use std::marker::PhantomData;

use crate::access::Access;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Present;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Optional;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Absent;

pub trait Presence {
    type OptionOrSelf: Presence;
    type Or<Other: Presence>: Presence;
    type Output<T>: Access<T>;

    fn or<T, Other: Presence>(
        self_: <Self as Presence>::Output<T>,
        other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T>;

    fn option_or_self<T>(
        option: Option<T>,
        self_: <Self as Presence>::Output<T>,
    ) -> <<Self as Presence>::OptionOrSelf as Presence>::Output<T>;
}

impl Presence for Present {
    type OptionOrSelf = Present;
    type Or<Other: Presence> = Present;
    type Output<T> = T;

    #[inline]
    fn or<T, Other: Presence>(
        self_: <Self as Presence>::Output<T>,
        _other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T> {
        self_
    }

    #[inline]
    fn option_or_self<T>(
        option: Option<T>,
        self_: <Self as Presence>::Output<T>,
    ) -> <<Self as Presence>::OptionOrSelf as Presence>::Output<T> {
        option.unwrap_or(self_)
    }
}

impl Presence for Optional {
    type OptionOrSelf = Optional;
    type Or<Other: Presence> = Other::OptionOrSelf;
    type Output<T> = Option<T>;

    #[inline]
    fn or<T, Other: Presence>(
        self_: <Self as Presence>::Output<T>,
        other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T> {
        Other::option_or_self(self_, other)
    }

    #[inline]
    fn option_or_self<T>(
        option: Option<T>,
        self_: <Self as Presence>::Output<T>,
    ) -> <<Self as Presence>::OptionOrSelf as Presence>::Output<T> {
        option.or(self_)
    }
}

impl Presence for Absent {
    type OptionOrSelf = Optional;
    type Or<Other: Presence> = Other;
    type Output<T> = PhantomData<T>;

    #[inline]
    fn or<T, Other: Presence>(
        _self: <Self as Presence>::Output<T>,
        other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T> {
        other
    }

    #[inline]
    fn option_or_self<T>(
        option: Option<T>,
        _self_: <Self as Presence>::Output<T>,
    ) -> <<Self as Presence>::OptionOrSelf as Presence>::Output<T> {
        option
    }
}

pub trait Project<To: Presence>: Presence {
    fn project<T>(value: Self::Output<T>) -> To::Output<T>;
}

pub trait TryProject<To: Presence>: Presence {
    fn try_project<T>(value: Self::Output<T>) -> Option<To::Output<T>>;
}

impl<From: Presence, To: Presence> TryProject<To> for From
where
    From: Project<To>,
{
    fn try_project<T>(value: From::Output<T>) -> Option<To::Output<T>> {
        Some(From::project(value))
    }
}

impl Project<Present> for Present {
    fn project<T>(value: T) -> T {
        value
    }
}

impl Project<Optional> for Present {
    fn project<T>(value: T) -> Option<T> {
        Some(value)
    }
}

impl Project<Absent> for Present {
    fn project<T>(_value: T) -> PhantomData<T> {
        PhantomData
    }
}

impl Project<Optional> for Optional {
    fn project<T>(value: Option<T>) -> Option<T> {
        value
    }
}

impl Project<Absent> for Optional {
    fn project<T>(_value: Option<T>) -> PhantomData<T> {
        PhantomData
    }
}

impl TryProject<Present> for Optional {
    fn try_project<T>(value: Option<T>) -> Option<T> {
        value
    }
}

impl Project<Absent> for Absent {
    fn project<T>(value: PhantomData<T>) -> PhantomData<T> {
        value
    }
}
