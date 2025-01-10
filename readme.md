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

Frequently Asked Questions
--------------------------

* *Why another random number generator crate?*

  I think the semi-official `rand` crate has some design issues that I can improve upon.

  The focus of this crate is on the ergonomics of the consumer side of randomness.

* *How does this crate improve on the `rand` crate?*

  - The `rand` crate overuses traits to provide its API to consumers. While traits are great they are not the easiest to work with, requiring figuring out which traits to import and on which types they are implemented.

    This crate uses inherent methods on the `Random` struct to provide a more ergonomic and consistent API. Looking for an RNG method? It's on the `Random` struct.

  - The `rand` crate puts its `thread_rng` front and center as it's the easiest way to generate randomness (through explicit use or `random` method).

    In my personal opinion thread local variables are not a good idea and should be avoided for the same reasons as global state.

    This crate uses explicit state management and seeds new PRNGs directly from the system's `getrandom`.

  - The `rand` crate has an inefficient implementation of generating unbiased uniform integers in a range.

    This crate uses a more efficient algorithm which avoids an expensive integer division. See the [benchmarks](benchmarks/rand/readme.md) against `rand` for details.

  - The `rand` crate's code is spread over several different crates which makes it harder to understand and contribute to.

    This crate is a single crate with a single focus: providing a better experience for consumers of randomness.

* *Which features of the `rand` crate are good?*

  The distribution trait and related types are fairly well designed and didn't need much change.

* *Which random number generators are implemented?*

  PRNG for non cryptographic use: `Xoshiro256` by [Sebastiano Vigna and David Blackman](http://prng.di.unimi.it/).

  Cryptographically secure PRNG: `ChaCha12` by [Daniel J. Bernstein](https://cr.yp.to/chacha/chacha-20080128.pdf).

  [`getrandom`](https://crates.io/crates/getrandom) as the source of system entropy.

* *Can I implement my own random number generator?*

  Yes, you can but it is not recommended. The chosen PRNGs are fast, well known and have good statistical properties.

  By not exposing consumers to the PRNG implementation details, the crate's API surface is kept small and simple.

* *The Rng traits are incompatible with `rand`, is this a problem?*

  No. The traits are only necessary to implement new Rngs but by design it is not recommended to do this. Consumers of randomness should use the `Random` struct and its methods.

* *Why are random floats generated in the half-open interval `[1.0, 2.0)` instead of `[0.0, 1.0)`?*

  Because it's very easy to generate a random float in that half open range (generate a random mantissa with a fixed exponent) and it avoids hard (design and implementation) questions.

  Converting to float and dividing by the integer range leaves a bias in the low bits of the float's mantissa. The same issue arises when subtracting `1.0` from a random float in the `[1.0, 2.0)` range (see [`examples/float_bias.rs`](examples/float_bias.rs)).

  The `Float01` distribution generates a true, unbiased random float in open interval `(0.0, 1.0)`.

* *Is it performant on 32-bit archs?*

  This crate is optimized for 64-bit archs with fast full 64-bit integer multiplication in mind. The same PRNGs are used on 32-bit archs to ensure compatibility and consistent behavior. This means the performance is not as good as on 64-bit archs.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
