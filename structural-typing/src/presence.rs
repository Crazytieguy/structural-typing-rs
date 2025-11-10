//! Type-level markers for field presence states.

use core::marker::PhantomData;

use crate::{access::Access, extract::Extract};

/// Marker indicating a field is present with a concrete value.
pub struct Present;
/// Marker indicating a field may or may not be present (Option).
pub struct Optional;
/// Marker indicating a field is absent (`PhantomData`).
pub struct Absent;

/// Trait for type-level presence markers with associated container types.
pub trait Presence {
    /// Presence when combined with Option.
    type OptionOrSelf: Presence;
    /// Result of merging this presence with another.
    type Or<Other: Presence>: Presence;
    /// Container type (T, Option\<T>, or `PhantomData`\<T>).
    type Output<T>: Access<T>
        + Extract<PhantomData<T>, T>
        + Extract<Option<T>, T>
        + Extract<Self::Output<T>, T>;

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

/// Infers presence state from value type: `T` → Present, `Option<T>` → Optional, `PhantomData<T>` → Absent.
pub trait InferPresence<T> {
    /// The presence state corresponding to this value type.
    type Presence: Presence<Output<T> = Self>;
}

impl<T> InferPresence<T> for T {
    type Presence = Present;
}

impl<T> InferPresence<T> for Option<T> {
    type Presence = Optional;
}

impl<T> InferPresence<T> for PhantomData<T> {
    type Presence = Absent;
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
