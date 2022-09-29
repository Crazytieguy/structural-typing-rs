#[macro_export]
macro_rules! partial {
    ($name:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        mod $name {
            #![allow(non_camel_case_types)]
            #![allow(type_alias_bounds)]

            pub struct Struct<$($field: $crate::access::Access<$ty> = $ty),+> {
                $(pub $field: $field),+
            }

            pub trait Interface: std::borrow::BorrowMut<Struct<$(Self::$field),+>> + Into<Struct<$(Self::$field),+>> {
                $(type $field: $crate::access::Access<$ty>;)+
            }

            impl<$($field: $crate::access::Access<$ty>),+> Interface for Struct<$($field),+> {
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
