µrandom
=======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/urandom.svg)](https://crates.io/crates/urandom)
[![docs.rs](https://docs.rs/urandom/badge.svg)](https://docs.rs/urandom)
[![Build status](https://github.com/CasualX/urandom/workflows/CI/badge.svg)](https://github.com/CasualX/urandom/actions)

Produce and consume randomness.

This crate provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.

This library is inspired by the semi-official [`rand`](https://crates.io/crates/rand) crate and an attempt to provide a better experience.

Usage
-----

Add this to your `Cargo.toml`:

```toml
[dependencies]
urandom = "0.2"
```

Quick Start
-----------

To get you started quickly, the easiest and highest-level way to get a random value is to use `urandom::new().next()`.
The `Random` struct provides a useful API on all Rngs, while the `distr` module provide further functionality on top of Rngs.

```rust
let mut rand = urandom::new();

// Generates a random boolean
if rand.coin_flip() {
	// Try printing a random unicode code point (probably a bad idea)!
	println!("char: {}", rand.next::<char>());
}

// Generates a float between 13.0 and 42.0
let y: f64 = rand.range(13.0..42.0);

// Shuffles the list of numbers
let mut numbers: Vec<i32> = (1..100).collect();
rand.shuffle(&mut numbers);
```

Features
--------

* `std` (default): Enable features that require the standard library.

  Without this feature, the crate can be used in `no_std` environments with limited functionality.

* `getrandom` (default): Use the `getrandom` crate to get random bytes from the operating system. This is the default source of randomness.

* `serde`: Enable serialization and deserialization support for the random number generators and distributions.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
