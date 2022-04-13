# Delegate
TODO update:
Tests that run regardless of `no_std` are under `any_std/`. The other two folders: `ok_std/` and `no_std/` re-import test from `any_std/`. Then `ok_std/` and `no_std/` have their own tests, too.

# Structure
Better have more granular module paths, and some name repetition in file paths and module names (like slices::bool_slice), rather than long files (source code).
