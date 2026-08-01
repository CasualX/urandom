//! Produce and consume randomness.
//!
//! This crate provides random number generators, distributions, sampling utilities, and randomness-related algorithms.
//!
//! It is a fork of the semi-official [`rand`](https://crates.io/crates/rand) crate, focused on providing a cohesive and ergonomic consumer API.
//!
//! # Quick Start
//!
#![cfg_attr(feature = "getrandom", doc = "The easiest way to get started is to create a [`Random`] generator with [`new`] and use its inherent methods.")]
#![cfg_attr(not(feature = "getrandom"), doc = "The easiest way to get started is to create a [`Random`] generator with [`seeded`] and use its inherent methods.")]
//!
//! The [`Random`] struct provides a convenient API over the random number generators, while the [`distr`] module provides distributions and sampling utilities.
//!
//! ```
#![cfg_attr(feature = "getrandom", doc = "let mut rand = urandom::new();")]
#![cfg_attr(not(feature = "getrandom"), doc = "let mut rand = urandom::seeded(42);")]
//!
//! // Roll a six-sided die.
//! let roll: u32 = rand.uniform(1..=6);
//!
//! // Choose a random element.
//! let colors = ["red", "green", "blue"];
//! let color = rand.choose(&colors).unwrap();
//!
//! // Shuffle a collection in place.
//! let mut numbers: Vec<_> = (1..=10).collect();
//! rand.shuffle(&mut numbers);
//!
//! println!("Rolled {roll}, chose {color}");
//! ```
//!
//! # Reproducibility policy
//!
//! This crate has two levels of reproducibility:
//!
#![cfg_attr(feature = "getrandom", doc = "* Deterministic generator types in [`rng`] and the [`new`] and [`seeded`] constructors have a strong guarantee across SemVer-compatible releases.")]
#![cfg_attr(not(feature = "getrandom"), doc = "* Deterministic generator types in [`rng`] and the [`seeded`] constructor have a strong guarantee across SemVer-compatible releases.")]
//!   Their generator algorithms, constructor choices and initialization, explicit seeding, and serialized state are stable.
//!
//! * Distributions in the [`distr`] module and sampling algorithms in [`Random`] are stable between patch releases of the same minor version, on a best-effort basis.

// Unsafe code is restricted throughout the crate except within the `rng` module
#![deny(unsafe_code)]

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod random;

pub mod rng;
pub mod distr;

pub use self::random::Random;
pub use self::rng::Rng;
pub use self::distr::Distribution;

#[cfg(feature = "dataview")]
mod dataview;

//----------------------------------------------------------------

/// Creates a new cryptographically secure pseudorandom number generator (CSPRNG).
///
/// The generator is seeded from the system entropy source.
/// Construct it once and reuse it to generate many values.
///
/// The generator choice and initialization behavior are covered by the
/// [reproducibility guarantee](crate::rng#reproducibility-guarantee).
///
/// # Examples
///
/// ```
/// let mut rand = urandom::new();
/// let value: i32 = rand.random();
/// ```
#[must_use]
#[inline]
#[cfg(feature = "getrandom")]
pub fn new() -> Random<rng::ChaCha12Rng> {
	rng::ChaCha12Rng::new()
}

/// Creates a reproducible non-cryptographic pseudorandom number generator with the given seed.
///
/// The seed does not need to look random. The generator's initialization handles degenerate seed values.
///
/// Given the same seed, its generator choice, initialization, and raw [`Rng`] output are covered by the
/// [reproducibility guarantee](crate::rng#reproducibility-guarantee).
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
