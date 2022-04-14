#![cfg_attr(feature = "no_std", no_std)]

// Do NOT use #[cfg(test)] under `any_std/src`, so that we can import & reuse it from `../ok_std/` and `../no_std_*/`.
pub mod slices;

