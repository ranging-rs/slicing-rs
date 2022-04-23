Abstracted slices in Rust.

# `std` and `no_std`/`no_std_vec` features
- `no_std` (plus optionally `no_std_vec` to support vector-based implementations & functions)
- `std` to indicate a need for full functionality (which is the same as `no_std_vec` plus `HashMap`-based).
- `no_std` (and `no_std_vec`) are mutually exclusive with `std`
- libraries can use `no_std_vec` feature for `Vec`-based functionality, and `std` for full functionality (including `HashMap`-based) with conditional compilation (controlled by their features). Then such libraries can be used in either `std` or `no_std` projects.

# Array size features
See features in [Cargo.toml](./Cargo.toml) and their respective [tests/](./tests/).

# Module name (and hence full qualified name) conventions
Some full qualified trait/struct/module names repeat their parts (hence there is some "stuttering").
