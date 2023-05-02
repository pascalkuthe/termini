# termini - minimal terminfo
[![crates.io](https://img.shields.io/crates/v/termini?style=flat-square)](https://crates.io/crates/termini)
![crates.io](https://img.shields.io/crates/l/termini?style=flat-square)

`termini` is a Rust library that provides access to the `terminfo` database.

Some highlights of `termini` include:

* supports extended capabilities
* easy to audit (single dependency, < 1k LOC)
* stability (extensively fuzzed to ensure absence of panics)
* tested on a wide array of `terminfo` databases

`termini`s main differentiating characteristic is that it's focused on providing a very minimal
functionality.
`termini` only has a single dependency (`home` to query the home directory) and has less than 1k LOC.
This means that it's easy to maintain/audit, doesn't introduce additional dependencies/compiletime and has
a smaller surface area for bugs.

`termini`s parser has been extensively fuzzed with `cargo-fuzz` to ensure that no panics occur even for fully malformed input.
Furthermore, `termini` is tested with a large array of compiled `terminifo` data to ensure it produces the correct results.

## Acknowledgements

During the implementation of this crate, the following code was used as reference:

* [terminfo](https://github.com/meh/rust-terminfo)
* [term](https://github.com/Stebalien/term)
* [cxterminfo](https://github.com/BxNiom/cx-terminfo)
* [pyterminfo](https://github.com/DirectXMan12/py-terminfo)
