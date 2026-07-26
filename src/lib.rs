/*!
Produce and consume randomness.

This crate provides random number generators, distributions, sampling utilities, and randomness-related algorithms.

It is a fork of the semi-official [`rand`](https://crates.io/crates/rand) crate, focused on providing a cohesive and ergonomic consumer API.

# Quick Start

The easiest way to get started is to create a [`Random`] generator with [`new`] and use its inherent methods.

The [`Random`] struct provides a convenient API over the random number generators, while the [`distr`] module provides distributions and sampling utilities.

```
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

# Reproducibility

Urandom treats the output of its deterministic random number generators as part of its stable API.
Unless an API documents otherwise, repeating the same sequence of RNG operations on a concrete generator initialized with the same seed
produces the same results across SemVer-compatible crate releases and supported targets.

See the [`rng`] module documentation for the generator guarantee and the [`distr`] module documentation
for the best-effort reproducibility policy of distributions and sampling algorithms.
*/

// Unsafe code is restricted throughout the crate except within the `rng` module
#![deny(unsafe_code)]

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod random;

pub mod rng;
pub mod distr;

pub use self::random::Random;
pub use self::rng::Rng;
pub use self::distr::Distribution;

//----------------------------------------------------------------

/// Creates a new non-cryptographic pseudorandom number generator (PRNG).
///
/// The generator is seeded from the system entropy source.
/// Construct it once and reuse it to generate many values.
///
/// Use [`csprng`] when outputs must be unpredictable.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::new();
/// let value: i32 = rand.random();
/// ```
#[must_use]
#[inline]
pub fn new() -> Random<rng::Xoshiro256Rng> {
	rng::Xoshiro256Rng::new()
}

/// Creates a reproducible non-cryptographic pseudorandom number generator with the given seed.
///
/// The seed does not need to look random. The generator's initialization handles degenerate seed values.
///
/// Unless otherwise documented, the same seed and sequence of RNG operations produces the same results
/// across SemVer-compatible releases of this crate and supported targets.
/// The underlying generator follows the [`rng`] module's reproducibility guarantee, while distributions and sampling algorithms
/// follow the [`distr`] module's best-effort reproducibility policy.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::seeded(42);
/// let value: i32 = rand.random();
/// assert_eq!(value, 368317477);
/// ```
#[must_use]
#[inline]
pub fn seeded(seed: u64) -> Random<rng::Xoshiro256Rng> {
	rng::Xoshiro256Rng::from_seed_u64(seed)
}

/// Hashes a value with Rust's [`DefaultHasher`](std::collections::hash_map::DefaultHasher).
///
/// This is a convenience function for obtaining a `u64` hash, for example to use as a seed.
///
/// The hash is deterministic for a given Rust version and target. However, the data fed by
/// [`Hash`](core::hash::Hash) is not guaranteed to be portable across targets or stable between
/// Rust versions, and the algorithm used by `DefaultHasher` may also change.
///
/// This function is not suitable for cryptographic use.
///
/// # Examples
///
/// ```
/// let seed = urandom::hash("example");
/// let mut rand = urandom::seeded(seed);
/// let value: i32 = rand.random();
/// ```
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn hash<T: ?Sized + core::hash::Hash>(value: &T) -> u64 {
	use core::hash::Hasher;
	let mut hasher = std::collections::hash_map::DefaultHasher::new();
	value.hash(&mut hasher);
	hasher.finish()
}

/// Creates a new cryptographically secure pseudorandom number generator (CSPRNG).
///
/// The generator is seeded from the system entropy source.
/// Construct it once and reuse it to generate many values.
///
/// This constructor uses [`rng::ChaCha12Rng`]. This choice is stable across SemVer-compatible releases.
/// See this [`rand` discussion](https://github.com/rust-random/rand/issues/932) for background on the choice of round count.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::csprng();
/// let value: i32 = rand.random();
/// ```
#[must_use]
#[inline]
pub fn csprng() -> Random<rng::ChaCha12Rng> {
	rng::ChaCha12Rng::new()
}
