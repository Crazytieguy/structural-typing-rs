pub trait Property {
    type Type;
}

pub struct P<T: Property>(pub T::Type);
