#![cfg_attr(feature = "no_std", no_std)]

// Do NOT use #[cfg(test)] under `any_std/src`, so that we can import & reuse it from `../ok_std/` and `../no_std_*/`.
pub mod bool_slice;
pub mod slices;

#[cfg(all(feature = "no_std", feature = "std"))]
compile_error!("std and no_std are mutually exclusive! Use maximum one of those two.");

#[cfg(all(not(feature = "no_std"), feature = "no_std_heap"))]
compile_error!("Use no_std_heap only together with no_std.");
