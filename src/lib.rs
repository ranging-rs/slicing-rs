#![cfg_attr(feature = "no_std", no_std)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(const_option_ext)]

pub mod abstra;
pub mod index;

pub mod bool_flag;
pub mod bool_slice;
pub mod byte_slice;
pub mod set;
pub mod slices;

#[cfg(all(feature = "no_std", feature = "std"))]
std_and_no_std_are_mutually_exclusive!();

#[cfg(all(not(feature = "no_std"), feature = "std"))]
pub mod hash;
