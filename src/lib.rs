#![cfg_attr(feature = "no_std", no_std)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]

pub mod bool_slice;
mod set;
mod slices;

pub mod abstra;
#[cfg(all(not(feature = "no_std"), feature = "std"))]
pub mod hash;
pub mod index;

pub use set::*;
pub use slices::*;

#[allow(unused)]
#[cfg(test)]
mod test {
    #[test]
    fn f() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
