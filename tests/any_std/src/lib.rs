#![feature(generic_arg_infer)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_option)]
#![feature(const_option_ext)]
// Do NOT use #[cfg(test)] under `any_std/src`, so that we can import & reuse it from `../ok_std/` and `../no_std/`.
pub mod slices;

struct WithGenericBeingNumber<const N: usize> {}

struct WithGenericBeingOption<const N: Option<usize>> {}

fn use_const_generic_types() {
    let number_based = WithGenericBeingNumber::<1> {}; //this works.

    // But all the below fail (with current nightly):
    let option_based = WithGenericBeingOption::<{ Some(2) }> {};
    let option_based = WithGenericBeingOption::<{ Some(3) }> {};
    let option_based = WithGenericBeingOption::<{ Some(4) }> {};
}
