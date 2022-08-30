#![allow(clippy::trait_duplication_in_bounds)]
#![allow(clippy::mismatching_type_param_order)]

pub trait Property {
    type Type;
}

pub struct P<T: Property>(pub T::Type);
