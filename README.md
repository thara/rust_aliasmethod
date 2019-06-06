aliasmethod
====

[![Build Status](https://travis-ci.org/thara/rust_aliasmethod.svg?branch=master)](https://travis-ci.org/thara/rust_aliasmethod)
[![](http://meritbadge.herokuapp.com/aliasmethod)](https://crates.io/crates/aliasmethod)

Implementation of [Walker's Alias method](https://en.wikipedia.org/wiki/Alias_method) by Rust.

The algorithm is principally useful when you need to random sampling with replacement by `O(1)`.

## Example

```rust
use rand::XorShiftRng;
use aliasmethod::AliasTable

let weights = vec![1.0, 1.0, 8.0];

let alias_table = AliasTable::new(weights)?;

let rng = XorShiftRng::from_seed([189522394, 1694417663, 1363148323, 4087496301]);
let n = alias_table.random(rng);

assert!(0 <= n && n <= weights.length);
```
