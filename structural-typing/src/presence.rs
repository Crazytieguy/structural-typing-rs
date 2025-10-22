//! Type-level markers for field presence states.

use std::marker::PhantomData;

use crate::access::Access;

/// Marker indicating a field is present with a concrete value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Present;
/// Marker indicating a field may or may not be present (Option).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Optional;
/// Marker indicating a field is absent (`PhantomData`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Absent;

/// Trait for type-level presence markers with associated container types.
pub trait Presence {
    /// The presence state when combined with an Option.
    type OptionOrSelf: Presence;
    /// Result of merging this presence with another.
    type Or<Other: Presence>: Presence;
    /// The container type for values with this presence (T, Option<T>, or `PhantomData`<T>).
    type Output<T>: Access<T>;

    /// Merge two values, preferring the first if present.
    fn or<T, Other: Presence>(
        self_: <Self as Presence>::Output<T>,
        other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T>;

    /// Convert an Option into the appropriate presence state.
    fn option_or_self<T>(
        option: Option<T>,
        self_: <Self as Presence>::Output<T>,
    ) -> <<Self as Presence>::OptionOrSelf as Presence>::Output<T>;
}

/// Trait for compile-time checked projection from one presence to another.
pub trait Project<To: Presence>: Presence {
    /// Project a value to the target presence state.
    fn project<T>(value: Self::Output<T>) -> To::Output<T>;
}

/// Trait for runtime checked projection that may fail.
pub trait TryProject<To: Presence>: Presence {
    /// Try to project a value to the target presence state.
    fn try_project<T>(value: Self::Output<T>) -> Option<To::Output<T>>;
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
