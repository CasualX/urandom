µrandom
=======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/urandom.svg)](https://crates.io/crates/urandom)
[![docs.rs](https://docs.rs/urandom/badge.svg)](https://docs.rs/urandom)
[![Build status](https://github.com/CasualX/urandom/actions/workflows/gate.yml/badge.svg)](https://github.com/CasualX/urandom/actions/workflows/gate.yml)

Produce and consume randomness.

This crate provides random number generators, distributions, sampling utilities, and randomness-related algorithms.

It is a fork of the semi-official [`rand`](https://crates.io/crates/rand) crate, focused on providing a cohesive and ergonomic consumer API.

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

// Roll a six-sided die.
let roll: u32 = rand.uniform(1..=6);

// Choose a random element.
let colors = ["red", "green", "blue"];
let color = rand.choose(&colors).unwrap();

// Shuffle a collection in place.
let mut numbers: Vec<_> = (1..=10).collect();
rand.shuffle(&mut numbers);

println!("Rolled {roll}, chose {color}");
```

Reproducibility
---------------

Generators created with `new()` and `csprng()` are independently seeded and expected to produce a different sequence each time. If system entropy is unavailable, construction panics.

To use a different entropy source, construct a concrete generator with its native seed using `from_seed(...)`.

For reproducible output, use `seeded(...)`

```rust
let mut a = urandom::seeded(42);
let mut b = urandom::seeded(42);

assert_eq!(a.random::<u64>(), b.random::<u64>());
```

For a fixed seed, the generated sequence is stable across compatible releases of urandom. A new major version may intentionally change generators, distributions, or algorithms and therefore produce different output.

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
