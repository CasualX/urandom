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
/// See [`Xoshiro256`](rng::Xoshiro256) for the concrete implementation.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::new();
/// let value: i32 = rand.random();
/// ```
#[must_use]
#[inline]
pub fn new() -> Random<impl Rng + Clone> {
	rng::Xoshiro256::new()
}

/// Creates a reproducible non-cryptographic pseudorandom number generator with the given seed.
///
/// The seed does not need to look random. The generator's initialization handles degenerate seed values.
///
/// Using the same seed and performing the same sequence of operations produces the same results across compatible versions of this crate and supported targets.
/// This guarantee extends to the distributions provided by this crate, except where their documentation notes target-dependent behavior.
///
/// See [`Xoshiro256`](rng::Xoshiro256) for the concrete implementation.
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
pub fn seeded(seed: u64) -> Random<impl Rng + Clone> {
	rng::Xoshiro256::from_seed(seed)
}

/// Creates a reproducible non-cryptographic pseudorandom number generator seeded from a hashable value.
///
/// The value is hashed with Rust's [`DefaultHasher`](std::collections::hash_map::DefaultHasher)
/// and the resulting hash is used to seed the generator.
///
/// The generated sequence is deterministic for a given Rust version and target. However, the
/// data fed by [`Hash`](core::hash::Hash) is not guaranteed to be portable across targets or
/// stable between Rust versions, and the algorithm used by `DefaultHasher` may also change.
///
/// This function is not suitable for cryptographic use.
///
/// See [`Xoshiro256`](rng::Xoshiro256) for the concrete implementation.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::seeded_hash("example");
/// let value: i32 = rand.random();
/// ```
#[cfg(feature = "std")]
#[must_use]
#[inline]
pub fn seeded_hash<T: ?Sized + core::hash::Hash>(value: &T) -> Random<impl Rng + Clone> {
	use core::hash::Hasher;
	let mut hasher = std::collections::hash_map::DefaultHasher::new();
	value.hash(&mut hasher);
	seeded(hasher.finish())
}

/// Creates a new cryptographically secure pseudorandom number generator (CSPRNG).
///
/// The generator is seeded from the system entropy source.
/// Construct it once and reuse it to generate many values.
///
/// See [`ChaCha12`](rng::ChaCha12) for the concrete implementation.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::csprng();
/// let value: i32 = rand.random();
/// ```
#[must_use]
#[inline]
pub fn csprng() -> Random<impl rng::SecureRng + Clone> {
	rng::ChaCha12::new()
}
