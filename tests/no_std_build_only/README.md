# Test `no_std` compatibility of dependencies
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