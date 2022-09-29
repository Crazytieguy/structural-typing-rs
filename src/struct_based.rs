use std::marker::PhantomData;

pub trait Access<T> {
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
    fn remove(self) -> Option<T>;
}

impl<T> Access<T> for PhantomData<T> {
    #[inline]
    fn get(&self) -> Option<&T> {
        None
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        None
    }
    #[inline]
    fn remove(self) -> Option<T> {
        None
    }
}

impl<T: Sized> Access<T> for T {
    #[inline]
    fn get(&self) -> Option<&T> {
        Some(self)
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        Some(self)
    }
    #[inline]
    fn remove(self) -> Option<T> {
        Some(self)
    }
}

impl<T> Access<T> for Option<T> {
    #[inline]
    fn get(&self) -> Option<&T> {
        self.as_ref()
    }
    #[inline]
    fn get_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }
    #[inline]
    fn remove(self) -> Option<T> {
        self
    }
}

#[macro_export]
macro_rules! partial {
    ($name:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        mod $name {
            #![allow(non_camel_case_types)]
            #![allow(type_alias_bounds)]

            pub struct Struct<$($field: $crate::struct_based::Access<$ty> = $ty),+> {
                $(pub $field: $field),+
            }

            pub trait Interface: std::borrow::BorrowMut<Struct<$(Self::$field),+>> + Into<Struct<$(Self::$field),+>> {
                $(type $field: $crate::struct_based::Access<$ty>;)+
            }

            impl<$($field: $crate::struct_based::Access<$ty>),+> Interface for Struct<$($field),+> {
                $(type $field = $field;)+
            }

            pub type Concrete<T: Interface> = Struct<$(T::$field),+>;

            pub mod has {
                $(pub trait $field: super::Interface<$field = $ty> {})+
                $(impl<T: super::Interface<$field = $ty>> $field for T {})+
            }
        }
    };
}
