# test-macro
[![Execute test](https://github.com/VeyronSakai/test-macro/actions/workflows/ci.yml/badge.svg)](https://github.com/VeyronSakai/test-macro/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/test-macro.svg)](https://crates.io/crates/test-macro)
[![Crates.io](https://img.shields.io/crates/d/test-macro.svg)](https://crates.io/crates/test-macro)
[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

## Overview

This is a macro for writing Rust test code in an easy way.

## Getting Started

Crate has to be added as a dev-dependency to Cargo.toml.

```toml
[dev-dependencies]
test-macro = "0.1.9"
```

## Example Usage

Consider testing the add function shown below.

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

In this case, if you want to write the test code in the tests directory, you can write it as follows.

```rust
// Need to import add function.
test_macro::test_assert_eq!(test_case_name, add(1, 2) => 3);
```

If you want to write it outside the tests directory, you can write it as follows.

```rust
#[cfg(test)]
mod tests {
    use test_macro::*;
    test_assert_eq!(test_case_name, add(1, 2) => 3);
}
```

This difference comes from importing the test-macro crate as a dev-dependencies crate.
