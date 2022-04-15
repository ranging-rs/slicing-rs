# Test `no_std` compatibility with `cargo build`.

After looking at https://crates.io/crates/trybuild, https://crates.io/crates/compiletest_rs and https://crates.io/crates/cargo-nono, this is the fastest & simplest way.

Beware that https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility uses no `libc`, and not just no `std`. That ends up with errors about missing `memcpy` and `setmem`.
