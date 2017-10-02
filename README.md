aliasmethod
====

[![Build Status](https://travis-ci.org/thara/rust_aliasmethod.svg?branch=master)](https://travis-ci.org/thara/rust_aliasmethod)
[![](http://meritbadge.herokuapp.com/aliasmethod)](https://crates.io/crates/aliasmethod)

Implementation of [Walker's Alias method](https://en.wikipedia.org/wiki/Alias_method) by Rust.

The algorithm is principally useful when you need to random sampling with replacement by `O(1)`.

## Example

```rust
use aliasmethod::{new_alias_table, alias_method}

let weights = vec![1.0, 1.0, 8.0];
match new_alias_table(weights) {
    Err(e) => {
        println!(false, "error : {}", e);
    }
    Ok(alias_table) => {
        let n = alias_method().random(&alias_table);
        assert!(0 <= n && n <= weights.length);
}
```
