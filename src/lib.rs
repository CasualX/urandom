/*!
Produce and consume randomness.

This crate provides utilities to generate random numbers, to convert them to useful types and distributions, and some randomness-related algorithms.

# Quick Start

To get you started quickly, the easiest and highest-level way to get a random value is to use `urandom::new().next()`.

The [`Random`] struct provides a useful API on all [`Rng`], while the [`distr`] module provide specific distributions on top of Rngs.

```
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

This library was inspired by the semi-official [`rand`](https://crates.io/crates/rand) crate and an attempt to provide a better experience.
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

/// Creates a new instance of the default pseudorandom number generator.
///
/// The generator is seeded securely from the system entropy source.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::new();
/// let value: i32 = rand.next();
/// ```
#[inline]
pub fn new() -> Random<impl Rng + Clone> {
	crate::rng::Xoshiro256::new()
}

/// Creates a new instance of the default pseudorandom number generator with the given seed.
///
/// The seed does not need to look random, the PRNG constructor ensures it can handle degenerate seed values.
///
/// This function guarantees that the same seed always produces the same sequence of randomness.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::seeded(42);
/// let value: i32 = rand.next();
/// assert_eq!(value, 368317477);
/// ```
#[inline]
pub fn seeded(seed: u64) -> Random<impl Rng + Clone> {
	crate::rng::Xoshiro256::from_seed(seed)
}

/// Creates a new cryptographically secure pseudorandom number generator.
///
/// The generator is seeded securely from the system entropy source.
///
/// # Examples
///
/// ```
/// let mut rand = urandom::csprng();
/// let value: i32 = rand.next();
/// ```
#[inline]
pub fn csprng() -> Random<impl Rng + Clone> {
	crate::rng::ChaCha12::new()
}
