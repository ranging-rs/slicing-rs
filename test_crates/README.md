# Test crates
Name of each subdirectory under [../test_crates] starts with `slicing_`. That way we can easily distinguish them in VS Code from similar subdirectories of `ranging-rs`[TODO web link] (when working on both).

Tests that are universal (regardless of `std/no_std`)  are in [../test_crates/slicing_any_std/]. Ones that are for `std` have paths starting with `test_crates/slicing_ok_std_`. `no_std` tests are in several directories with paths starting with `test_crates/slicing_no_std_`. All `std` and `no_std` test crates re-use parts of [../test_crates/slicing_any_std/].

Various `std` and `no_std` test crates also have their own tests (on top of tests from [../test_crates/slicing_any_std/]).

## no_std test crates
`no_std` integration tests in [../test_crates/] have project names
- starting with `slicing_no_std_basic_` with no `Vec` (`core::collections::Vec), or
- starting with `slicing_no_std_vec_` with `Vec` (`core::collections::Vec`), and
- ending with `_build` for testing build only, or
- ending with `_run` for running tests only (as if in `no_std`, but using your default target).

## Testing features and platforms
The middles names of all `no_std` project names (having paths starting with `test_crates/slicing_no_std_`), and last names of all `std` project names (having paths starting with `test_crates/slicing_ok_std_`) indicate what crate features are being tested.

Some features are mutually exclusive. But we want all test crates to be in the main workspace (as defined in [Cargo.toml]), so that they can share built targets (as much as possible). We accomplish that by using different built targets for mutually exclusive features. This way we also test build (and some execution) of multiple targets.

