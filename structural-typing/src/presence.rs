//! Type-level markers for field presence states.

use core::marker::PhantomData;

use crate::access::Access;

/// Marker indicating a field is present with a concrete value.
pub struct Present;
/// Marker indicating a field may or may not be present (Option).
pub struct Optional;
/// Marker indicating a field is absent (`PhantomData`).
pub struct Absent;

/// Trait for type-level presence markers with associated container types.
pub trait Presence {
    /// The presence state when combined with an Option.
    type OptionOrSelf: Presence;
    /// Result of merging this presence with another.
    type Or<Other: Presence>: Presence;
    /// What remains from `Source` when extracting `Self`. `Present/Optional` → `Absent`; `Absent` → `Source`.
    type RemainderFrom<Source: Presence>: Presence;
    /// The container type for values with this presence (T, Option\<T>, or `PhantomData`\<T>).
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
    type RemainderFrom<Source: Presence> = Absent;
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
    type RemainderFrom<Source: Presence> = Absent;
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
    type RemainderFrom<Source: Presence> = Source;
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
