# The Pavex Diary

This is my [Pavex](https://pavex.dev) learning journal.

2025-02-24: Initial commit

- scaffold with `pavex new --template="quickstart" pavex-diary`
- add `justfile` configuration basics, [just](https://just.systems/)
- add MIT `License.txt`
- start this `README.md`
- set up workspace configuration (see note from Luca below)
  - > If that's the case, you need to (manually) bump app's dependency in the server SDK crate Cargo.toml to 0.2.0.
  - > This is unfortunately a chicken and egg problem—for cargo px to run, we need to compute cargo metadata for the dep tree. But if we can't run cargo px, we can't update the generated crate, so we can't fix the TOML.
- set up [bacon](https://dystroy.org/bacon) `bacon --init`, modify it to use `just` commands instead of the defaults
