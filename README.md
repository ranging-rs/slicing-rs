# Purpose
Abstracted slices in Rust.

# Main principles
## API and source
- Explicit and clear. For example, `std` and `no_std` features are mutually exclusive, so users detect any conflicts as early as possible.
- Ergonomic: Simple (rather than exotic but complex). For example:
-- TODO Hashed could be a value object (transparent wrapper). It would make it more "future-proof"/flexible, but not backwards-compatible with your/third party's existing API's. Implementing it directly for `std::collections::HashMap` (when using `std` feature) makes it a drop-in replacement for `HashMap` - easy to use.
-- `size_for_array_only`, `allow_empty_arrays`, `disable_empty_arrays` are crate features, rather than per-struct or per-object fields.
- Generic: Generics, constant generic parameters and associated generic types are essential for memory efficiency and type safety. The drawbacks are longer build times and larger binaries. We optimize it where worthwhile.

## Requiring Nightly Rust
As of mid 2022, we need Rust nightly (for const time generics and other language features). Otherwise this couldn't be so ergonomic.

## Supporting no_std
We test building for several of more supported `no_std` Rust targets. See `no_std` at
https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2.

# Features
## std, no_std and no_std_vec
- `no_std` (plus optionally `no_std_vec` to support vector-based implementations & functions)
- `std` to indicate a need for full functionality (which is the same as `no_std_vec` plus `HashMap`-based).
- `no_std` (and `no_std_vec`) are mutually exclusive with `std`
- libraries can use `no_std_vec` feature for `Vec`-based functionality, and `std` for full functionality (including `HashMap`-based) with conditional compilation (controlled by their features). Then such libraries can be used in either `std` or `no_std` projects.

## Array size features
See features in [Cargo.toml](./Cargo.toml) and their respective [tests/](./tests/).

# Module name (and hence full qualified name) conventions
Some full qualified trait/struct/module names repeat their parts. Better have more granular module paths, and some name repetition in file paths and module names (like `slices::bool_slice`), rather than long (source) files).
