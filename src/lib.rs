// @TODO auto-include RustDoc from ../README.md.

// @TODO remove & cleanup later
#![allow(unused_variables)]
//
#![cfg_attr(feature = "no_std", no_std)]
//
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(const_option_ext)]
#![feature(array_from_fn)]

pub mod abstra;
pub mod index;

pub mod byte_slice;
pub mod slices;

#[cfg(all(feature = "no_std", feature = "std"))]
compile_error!("std and no_std are mutually exclusive! Use maximum one of those two.");

#[cfg(all(not(feature = "no_std"), feature = "no_std_vec"))]
compile_error!("Use no_std_vec only together with no_std.");

#[cfg(all(feature = "allow_empty_arrays", feature = "disable_empty_arrays"))]
compile_error!("allow_empty_arrays and disable_empty_arrays are mutually exclusive! Use maximum one of those two.");
