use std::marker::PhantomData;

use crate::access::Access;

/// Marker type indicating a field is known to be present at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Present;

/// Marker type indicating a field may or may not be present (runtime check needed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Optional;

/// Marker type indicating a field is known to be absent at compile-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Absent;

/// Trait for presence states that might contain a value (Present or Optional).
pub trait MaybePresent: Presence {}
impl MaybePresent for Present {}
impl MaybePresent for Optional {}

/// Core trait for tracking field presence at the type level.
///
/// This trait defines how presence states combine and what types they produce.
pub trait Presence {
    /// The presence state when combined with an Option.
    type OptionOrSelf: Presence;

    /// The presence state when combined with another presence state.
    type Or<Other: Presence>: Presence;

    /// The actual Rust type for a field with this presence state.
    type Output<T>: Access<T>;

    /// Combines two presence states, preferring the left value when both are present.
    fn or<T, Other: Presence>(
        self_: <Self as Presence>::Output<T>,
        other: Other::Output<T>,
    ) -> <<Self as Presence>::Or<Other> as Presence>::Output<T>;

    /// Combines an Option with this presence state.
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
