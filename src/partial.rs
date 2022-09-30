#[macro_export]
macro_rules! partial {
    ($name:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        mod $name {
            #![allow(non_camel_case_types)]
            #![allow(type_alias_bounds)]
            use $crate::access::Access;

            #[derive(Debug)]
            pub struct Struct<$($field: $crate::access::Access<$ty> = $ty),+> {
                $(pub $field: $field),+
            }

            pub trait Interface: ::std::borrow::BorrowMut<Struct<$(Self::$field),+>> + Into<Struct<$(Self::$field),+>> {
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

            impl<T: Interface, U: Interface> $crate::merge::Merge<U> for T {
                type Output = Struct<
                    $(<U::$field as $crate::access::Access<$ty>>::Or<T::$field>),+
                >;
                fn merge(self, other: U) -> Self::Output {
                    let concrete = self.into();
                    let other = other.into();
                    Struct {
                        $($field: other.$field.or(concrete.$field)),+
                    }
                }
            }

            pub fn empty() -> Struct<$(::std::marker::PhantomData<$ty>),+> {
                Struct {
                    $($field: ::std::marker::PhantomData),+
                }
            }
        }
    };
}
