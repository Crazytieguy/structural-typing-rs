#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(core_intrinsics)]
#![feature(const_trait_impl)]
#![feature(const_type_id)]
#![feature(generic_const_exprs)]
#![warn(clippy::pedantic)]

mod name_based;
mod type_based;
mod type_id_based;

#[cfg(test)]
mod tests {}
