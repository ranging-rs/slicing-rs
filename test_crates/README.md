# Test crates
## Test crate names
Name of each subdirectory (under this directory: `test_crates`) starts with `slicing_`. That way we can easily distinguish them in VS Code from similar subdirectories of `ranging-rs`[TODO web link] (when working on both).

## # Test crate name prefixes
Tests that are universal (regardless of `std` or `no_std`)  are in [slicing_any_std/](slicing_any_std). Ones that are for `std` have paths starting with `slicing_ok_std_`. `no_std` tests are in several directories with paths starting with `slicing_no_std_`.

Various `std` and `no_std` test crates re-use parts of [slicing_any_std](slicing_any_std). They also have their own tests (on top of tests from slicing_any_std](slicing_any_std)).

## no_std test crates
`no_std` integration tests in have project names
- starting with `slicing_no_std_basic_`: with no `Vec` (`core::collections::Vec), or
- starting with `slicing_no_std_vec_`: with `Vec` (`core::collections::Vec`), and
- ending with `_build`: for testing build only, or
- ending with `_run`: for running tests only (as if in `no_std`, but using your default target).

Even though these crates have long names, we don't put them under any subdirectory structure. (As per TODO 'Test crates' above.) Then we can differentiate their names (in VS Code).

## Testing features and platforms
The middles names of all `no_std` project names (parts right of `slicing_no_std_` and left of `_build` or `_run`), and last names of all `std` project names (postfixes right of `slicing_ok_std_`) indicate what crate features are being tested.

Some features are mutually exclusive. But we want all test crates to be in the main workspace (as defined in [Cargo.toml]). That re-uses some processing (macro expansion). Also, it we see any warnings/errors without repetitions. We accomplish that by using different built targets for mutually exclusive features. This way we also test build (and some execution) of multiple targets.

