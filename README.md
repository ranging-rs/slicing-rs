Abstracted slices in Rust.

# `std` and `no_std`/`no_std_vec` features
- `no_std` (plus optionally `no_std_vec` to support vector-based implementations & functions)
- `std` to indicate a need for full functionality (which is the same as `no_std_vec` plus `HashMap`-based).
- `no_std` (and `no_std_vec`) are mutually exclusive with `std`
- `std/no_std`-agnostic libraries can use `Vec`-based and `HashMap`-based functionality with conditional compilation. Then such libraries can be used in either `std` or `no_std` project.

# Array size features
See features in [Cargo.toml](./Cargo.toml) and their respective [tests/](./tests/).

# Module name (and hence full qualified name) conventions
Some full qualified trait/struct/module names repeat their parts (hence there is some "stuttering").

# Test conventions
Tests of any public functionality are under `tests/`. There are also test modules under (`slicing`'s top level) [src/](./src/). Howevever, those are for testing private functionality only.

Folder & crate names under [tests/](./tests/) start with `slicing_`. That is to differentiate them (as project folders) in VS Code (as compared to similar test folders in the related `ranging` project).