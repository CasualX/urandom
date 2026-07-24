µrandom
=======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/urandom.svg)](https://crates.io/crates/urandom)
[![docs.rs](https://docs.rs/urandom/badge.svg)](https://docs.rs/urandom)
[![Build status](https://github.com/CasualX/urandom/workflows/CI/badge.svg)](https://github.com/CasualX/urandom/actions)

Produce and consume randomness.

This crate provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.

This library is a fork of the semi-official [`rand`](https://crates.io/crates/rand) crate and an attempt to provide a better experience.

Usage
-----

Add this to your `Cargo.toml`:

```toml
[dependencies]
urandom = "1.0"
```

Quick Start
-----------

To get you started quickly, the easiest and highest-level way to get a random value is to use `urandom::new().random()`.
The `Random` struct provides a convenient API over the random number generators, while the `distr` module provides distributions and sampling utilities.

```rust
let mut rand = urandom::new();

// Generates a random boolean
if rand.coin_flip() {
	// Try printing a random Unicode code point (probably a bad idea)!
	println!("char: {}", rand.random::<char>());
}

// Generates a float between 13.0 and 42.0
let y: f64 = rand.uniform(13.0..42.0);

// Shuffles the list of numbers
let mut numbers: Vec<i32> = (1..100).collect();
rand.shuffle(&mut numbers);
```

Reproducibility
---------------

Generators created with `new()` and `csprng()` are seeded from system entropy and produce a different sequence each time they are constructed. If system entropy is unavailable, construction panics.

For reproducible output, use `seeded(...)`

```rust
let mut a = urandom::seeded(42);
let mut b = urandom::seeded(42);

assert_eq!(a.random::<u64>(), b.random::<u64>());
```

For a fixed seed or key, the generated sequence is stable across compatible releases of urandom. A new major version may intentionally change generators, distributions, or algorithms and therefore produce different output.

The quick constructors return opaque generator types. This keeps common usage simple and allows urandom to select an appropriate implementation. When a concrete generator type must be named, such as when storing it in a struct or serializing it, select a backend from the rng module directly.

Features
--------

* `std` (default): Enables features that require the standard library.

  Without this feature, the crate can be used in `no_std` environments with limited functionality. See the API documentation for details.

* `serde`: Enables serialization and deserialization for random number generators and distributions.

  Deserializing a generator state resumes the same random sequence from the point at which it was saved.

System entropy is currently provided by `getrandom`. Custom entropy sources rely on [`getrandom`'s custom-backend mechanism](https://docs.rs/getrandom/0.3/#custom-backend). This integration is not part of urandom's stable API and may change between otherwise compatible releases.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
