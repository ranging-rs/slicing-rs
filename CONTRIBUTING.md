# Test conventions
Tests of any public functionality are under `test_crates/`. Folder & crate names
under [test_crates/](./tests/) start with `slicing_`. That is to differentiate
them (as project folders) in VS Code (as compared to similar test folders in the
related `ranging-rs` project).

See also https://peter-kehl.github.io/embedded_low_level_rust.

There are also test
modules under (`slicing`'s top level) [src/](./src/). Howevever, those are for
testing private functionality only.

# presentation anchors in inline comments
Sections of source files are loaded by
https://peter-kehl.github.io/embedded_low_level_rust. Instead of line numbers,
that presentation refers to parts of the source files by strings that are
present in code inline comments. Those comments serve as delimiters of code
sections to present. So please leave in any Rust comments `//` or `Cargo.toml`
comments `#` containing `presentation-`.