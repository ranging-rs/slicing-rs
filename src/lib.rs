#![cfg_attr(feature = "no_std", no_std)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]

pub mod abstra;
pub mod index;

pub mod bool_flag;
pub mod bool_slice;
pub mod byte_slice;
pub mod set;
pub mod slices;

#[cfg(all(not(feature = "no_std"), feature = "std"))]
pub mod hash;
