# Test conventions
Tests of any public functionality are under `test_crates/`. Folder & crate names
under [test_crates/](./tests/) start with `slicing_`. That is to differentiate
them (as project folders) in VS Code (as compared to similar test folders in the
related `ranging-rs` project).

See also https://peter-kehl.github.io/embedded_low_level_rust.

There are also test modules under (`slicing`'s top level) [src/](./src/).
Howevever, those are for testing private functionality only.

# Presentation anchors in inline comments
Sections of source files are loaded by
https://peter-kehl.github.io/embedded_low_level_rust. Instead of line numbers,
that presentation refers to parts of the source files by strings that are
present in the code in inline comments. Those comments serve as delimiters of
code sections to present. So please leave in any Rust comments `//` or /*...*/
or `Cargo.toml` comments `#` containing `presentation-`.

## License

This project is license under either of
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
at your option.

## Contributing
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
 slicing-rs by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.