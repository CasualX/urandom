/*!
Produce and consume randomness.

This crate provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.

This library is inspired by the semi-official [`rand`](https://crates.io/crates/rand) crate and an attempt to provide a better experience.

# Quick Start

To get you started quickly, the easiest and highest-level way to get a random value is to use `urandom::new().random()`.

The [`Random`] struct provides a useful API on all [`Rng`], while the [`distr`] module provide specific distributions on top of Rngs.

```
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
*/

// Unsafe code is restricted to certain specific Rng implementations
#![deny(unsafe_code)]

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod random;

pub mod rng;
pub mod distr;

pub use self::rng::Rng;
pub use self::distr::Distribution;
pub use self::random::Random;

//----------------------------------------------------------------

/// Creates a new instance of the default PRNG.
///
/// The generator is seeded from the system entropy source.
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
	crate::rng::Xoshiro256::new()
}

/// Creates a new instance of the default PRNG with the given seed.
///
/// The seed does not need to look random, the PRNG constructor ensures it can handle degenerate seed values.
///
/// The same seed and sequence of operations produces the same results across compatible crate versions and supported targets.
/// This guarantee extends to the distributions provided by this crate, except where their documentation notes target-dependent behavior.
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
	crate::rng::Xoshiro256::from_seed(seed)
}

/// Creates a new non-cryptographic PRNG seeded by a hashable value.
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

/// Creates a new cryptographically secure PRNG.
///
/// The generator is seeded from the system entropy source.
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
	crate::rng::ChaCha12::new()
}
