//! Extract selected fields from structs, returning the selected fields and remainder.
//!
//! Use `extract()` when extraction always succeeds (e.g., `Present` → `Optional`).
//! Use `try_extract()` when extraction may fail (e.g., `Optional` → `Present` can fail if `None`).

use core::marker::PhantomData;

use crate::access::Access;

/// Trait for compile-time checked extraction from one access type to another.
///
/// Returns both the extracted value (in target access type) and the remainder.
pub trait Extract<A: Access<T>, T>: Access<T> + Sized + TryExtract<A, T> {
    /// Extract a value into target type and remainder parts.
    fn extract(self) -> (A, <A as Access<T>>::RemainderFrom<Self>);
}

/// Trait for runtime checked extraction that may fail.
///
/// Returns Ok with (extracted, remainder) on success, or Err with original value on failure.
///
/// Most conversions delegate to `Extract` and always succeed; only `Option<T>` → `T` can fail when the value is `None`.
pub trait TryExtract<A: Access<T>, T>: Access<T> + Sized {
    /// Try to extract a value into target type and remainder parts.
    ///
    /// # Errors
    ///
    /// Returns `Err` with the original value if the extraction cannot be performed
    /// (e.g., when trying to convert `Option<T>` to `T` but the value is `None`).
    fn try_extract(self) -> Result<(A, <A as Access<T>>::RemainderFrom<Self>), Self>;
}

impl<T, A: Access<T>> Extract<PhantomData<T>, T> for A
where
    Self: TryExtract<PhantomData<T>, T>,
{
    fn extract(
        self,
    ) -> (
        PhantomData<T>,
        <PhantomData<T> as Access<T>>::RemainderFrom<Self>,
    ) {
        (PhantomData, self)
    }
}

impl<T, A: Access<T>> Extract<Option<T>, T> for A
where
    Self: TryExtract<Option<T>, T>,
{
    fn extract(self) -> (Option<T>, <Option<T> as Access<T>>::RemainderFrom<Self>) {
        (self.into_option(), PhantomData)
    }
}

impl<T> Extract<T, T> for T {
    fn extract(self) -> (T, <T as Access<T>>::RemainderFrom<Self>) {
        (self, PhantomData)
    }
}

impl<T, A: Access<T>> TryExtract<A, T> for Option<T> {
    fn try_extract(self) -> Result<(A, <A as Access<T>>::RemainderFrom<Self>), Self> {
        A::try_from_option(self)
    }
}

impl<T, A: Access<T>> TryExtract<A, T> for T {
    fn try_extract(self) -> Result<(A, <A as Access<T>>::RemainderFrom<Self>), Self> {
        Ok(A::from_value(self))
    }
}

impl<T> TryExtract<Option<T>, T> for PhantomData<T> {
    fn try_extract(
        self,
    ) -> Result<(Option<T>, <Option<T> as Access<T>>::RemainderFrom<Self>), Self> {
        Ok((None, self))
    }
}

impl<T> TryExtract<PhantomData<T>, T> for PhantomData<T> {
    fn try_extract(
        self,
    ) -> Result<
        (
            PhantomData<T>,
            <PhantomData<T> as Access<T>>::RemainderFrom<Self>,
        ),
        Self,
    > {
        Ok((PhantomData, self))
    }
}
