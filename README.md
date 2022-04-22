Abstracted slices in Rust.

# `std` and `no_std` features
- `no_std` (plus optionally `no_std_vec` to support vector-based implementations & functions)
- `std` for full functionality

# Module name (and hence full qualified name) conventions
Some full qualified trait/struct/module names repeat their parts (hence there is some "stuttering").

# Test conventions
Tests of any public functionality are under `tests/`. Any test modules under (`slicing`'s top level) `src/` are for testing private functionality only.