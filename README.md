Abstracted slices in Rust.

# `std` and `no_std` features
- `no_std` (plus optionally `no_std_vec` to support vector-based implementations & functions)
- `std` for full functionality

# Module name (and hence full qualified name) conventions
Some full qualified trait/struct/module names repeat their parts (hence there is some "stuttering").

# Test conventions
Tests of any public functionality are under `tests/`. There are also test modules under (`slicing`'s top level) `src/`. Howevever, those are for testing private functionality only.

Folder & crate names under `tests/` start with `slicing_`. That is to differentiate them (as project folders) in VS Code (as compared to similar test folders in the related `ranging` project).