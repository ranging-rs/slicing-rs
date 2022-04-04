# Test `no_std` build compatibility (but don't run tests themselves).

After looking at https://crates.io/crates/trybuild, https://crates.io/crates/compiletest_rs and https://crates.io/crates/cargo-nono, this is the fastest way.

Run: `cargo rustc -- -C link-arg=-nostartfiles`

For GitHub Actions use
```
name: Ensure that crate is no_std
  uses: actions-rs/cargo@v1
  with:
    command: rustc
    args: --manifest-path=ensure_no_std/Cargo.toml -- -C link-arg=-nostartfiles
```

The above comes from https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility.