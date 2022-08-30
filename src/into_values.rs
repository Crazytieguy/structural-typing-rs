use crate::{Access, Property, P};

pub trait IntoValues<T>: Access {
    fn _into_values(self) -> T;
}

impl<A> IntoValues<(A::Type,)> for (P<A>,)
where
    A: Property,
{
    fn _into_values(self) -> (A::Type,) {
        (self.0 .0,)
    }
}

impl<A, B> IntoValues<(A::Type, B::Type)> for (P<A>, P<B>)
where
    A: Property,
    B: Property,
{
    fn _into_values(self) -> (A::Type, B::Type) {
        (self.0 .0, self.1 .0)
    }
}

impl<A, B, C> IntoValues<(A::Type, B::Type, C::Type)> for (P<A>, P<B>, P<C>)
where
    A: Property,
    B: Property,
    C: Property,
{
    fn _into_values(self) -> (A::Type, B::Type, C::Type) {
        (self.0 .0, self.1 .0, self.2 .0)
    }
}

impl<A, B, C, D> IntoValues<(A::Type, B::Type, C::Type, D::Type)> for (P<A>, P<B>, P<C>, P<D>)
where
    A: Property,
    B: Property,
    C: Property,
    D: Property,
{
    fn _into_values(self) -> (A::Type, B::Type, C::Type, D::Type) {
        (self.0 .0, self.1 .0, self.2 .0, self.3 .0)
    }
}
