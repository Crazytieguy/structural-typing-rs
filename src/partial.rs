#[macro_export]
macro_rules! partial {
    ($name:ident { $($field:ident: $ty:ty),+ $(,)? }) => {
        mod $name {
            #![allow(non_camel_case_types)]
            #![allow(type_alias_bounds)]
            use $crate::presence::Presence;

            pub trait Fields {
                $(type $field: Presence;)+
            }

            pub struct Empty;
            impl Fields for Empty {
                $(type $field = $crate::presence::Absent;)+
            }

            pub struct Struct<F: Fields = Empty> {
                $(pub $field: <<F as Fields>::$field as Presence>::Output<$ty>),+
            }

            impl Struct {
                pub fn empty() -> Self {
                    Self {
                        $($field: ::std::marker::PhantomData),+
                    }
                }
            }

            pub struct Merge<F1: Fields, F2: Fields>(::std::marker::PhantomData<(F1, F2)>);

            impl<F1: Fields, F2: Fields> Fields for Merge<F1, F2> {
                $(type $field = <<F2 as Fields>::$field as Presence>::Or<<F1 as Fields>::$field>;)+
            }

            impl<F1: Fields> Struct<F1> {
                pub fn merge<F2: Fields>(self, other: Struct<F2>) -> Struct<Merge<F1, F2>> {
                    Struct {
                        $($field: <<F2 as Fields>::$field as Presence>::or(other.$field, self.$field)),+
                    }
                }
            }
        }
    };
}
