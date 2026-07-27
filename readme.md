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

Reproducibility policy
----------------------

Generators created with `new()` and `csprng()` are independently seeded and expected to produce a different sequence each time. If system entropy is unavailable, construction panics.

To use a different entropy source, construct a concrete generator with its native seed using `from_seed(...)`.

For reproducible output, use `seeded(...)`:

```rust
let mut a = urandom::seeded(42);
let mut b = urandom::seeded(42);

assert_eq!(a.random::<u64>(), b.random::<u64>());
```

Urandom provides two levels of reproducibility:

* Concrete deterministic generators in the `rng` module have a strong guarantee across SemVer-compatible releases.
  Given the same explicit seed or state and the same sequence of `Rng` calls and arguments, they produce the same
  output on supported targets. Their algorithms, seed expansion, and method behavior are not changed, including
  for performance improvements. With the `serde` feature, saved generator state remains readable and resumes the
  same stream at the saved position.

  The calls must match exactly: one `next_u64()` is not equivalent to two `next_u32()` calls or `next_f64()` call.
  Construction from system entropy is intentionally non-deterministic.

* Distributions, sampling algorithms, and root-level convenience constructors such as `seeded()` preserve their
  observable behavior between patch releases of the same minor version, on a best-effort basis. They may change
  in a new minor release. Integer-only sampling is generally predictable across supported targets, while
  floating-point results can depend on the platform, especially for math-heavy distributions. Uniform floating-point
  sampling is predictable on targets with Rust's strict IEEE 754 floating-point semantics.

For reproducible recordings, simulations, or game replays, pin urandom to one minor release (for example, `~1.0`)
and keep external inputs and platform behavior consistent. When the generator stream itself must remain stable
across minor releases, construct a concrete type from the `rng` module directly.

Features
--------

* `std` (default): Enables features that require the standard library.

  Without this feature, the crate can be used in `no_std` environments with limited functionality. See the API documentation for details.

* `serde`: Enables serialization and deserialization for random number generators and distributions.

  Serialized generator state is covered by the strong generator guarantee described above.

System entropy is currently provided by `getrandom`. Custom entropy sources rely on [`getrandom`'s custom-backend mechanism](https://docs.rs/getrandom/0.3/#custom-backend).
This integration is not part of urandom's stable API and may change between otherwise SemVer-compatible releases.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
