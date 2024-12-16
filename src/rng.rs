/*!
Random number generators.

Pseudorandom number generators
-------------------------------

These generators implement fast PRNG which are suitable for normal use in non-cryptography applications.

* [`Xoshiro256`] Rng:

  Kindly taken from [Sebastiano Vigna](http://vigna.di.unimi.it/)'s excellent [PRNG shootout](http://prng.di.unimi.it/) article.

* [`SplitMix64`] Rng:

  Simple 64-bit state generator, useful for seeding other generators.

* [`Wyrand`] Rng:

  Tiny and very fast pseudorandom number generator based on [rapidhash](https://github.com/Nicoshev/rapidhash).

Cryptographically secure generators
-----------------------------------

These generators implement suitable CSPRNG implementations.

* [`ChaCha8`], [`ChaCha12`], [`ChaCha20`] Rng:

  Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.

  The current algorithm used by the default csprng is ChaCha12.
  Please see this relevant [rand issue](https://github.com/rust-random/rand/issues/932) for the discussion.
  This may change as new evidence of cipher security and performance becomes available.

* [`System`] Rng:

  Randomness directly from the system entropy source.

  For performance reasons this generator fetches entropy in blocks of `N` 32-bit words.
  The bigger `N` is, the less often the system entropy source is called.

Other generators
----------------

* [`Mock`] Rng:

  Mocks the Rng. Produces randomness directly from the given iterator and panics when it runs out of items.

* [`Read`] Rng:

  Read randomness from files and others with the `std::io::Read` trait.

*/

#![allow(unsafe_code)]

use core::{mem, ptr, slice};
use core::mem::MaybeUninit;

use crate::Random;

pub(crate) mod util;

/// Random number generator interface.
pub trait Rng {
	/// Returns the next `u32` in the sequence.
	fn next_u32(&mut self) -> u32;

	/// Returns the next `u64` in the sequence.
	fn next_u64(&mut self) -> u64;

	/// Returns a uniform random `f32` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 23 bits are necessary to construct a random float in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// The default implementation simply gets its random bits from `next_u32`.
	#[inline]
	fn next_f32(&mut self) -> f32 {
		util::rng_f32(self.next_u32())
	}

	/// Returns a uniform random `f64` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 52 bits are necessary to construct a random double in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// The default implementation simply gets its random bits from `next_u64`.
	#[inline]
	fn next_f64(&mut self) -> f64 {
		util::rng_f64(self.next_u64())
	}

	/// Fills the byte slice with uniform random bytes.
	///
	/// Implementations are required to produce the same result regardless of endianness.
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]);

	/// Advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	fn jump(&mut self);
}

/// Marker trait for cryptographically secure random number generators.
pub trait SecureRng : Rng {}

/// Constructors for deterministic random number generators.
pub trait SeedRng: Rng + Sized {
	/// Creates a new instance seeded securely from system entropy.
	///
	/// This method is the recommended way to construct PRNGs since it is convenient and secure.
	///
	/// # Panics
	///
	/// If [`getentropy`] is unable to provide secure entropy this method will panic.
	fn new() -> Random<Self>;

	/// Creates a new PRNG seeded from another `Rng`.
	///
	/// This may be useful when needing to rapidly seed many PRNGs from a master PRNG, and to allow forking of PRNGs.
	///
	/// The master PRNG should use a sufficiently different algorithm from the child PRNG (ideally a CSPRNG) to avoid correlations between the child PRNGs.
	fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<Self>;

	/// Creates a new PRNG using the given seed.
	///
	/// The seed is not required to look random, the PRNG constructor must ensure it can handle degenerate seed values by mixing it first.
	/// The PRNG constructor should not panic on degenerate seed values (such as zero) and instead replace it with something else.
	///
	/// This **is not suitable for cryptography**, as should be clear given that the input size is only 64 bits.
	///
	/// Implementations are required to be reproducible given the same seed.
	/// _Changing_ the implementation of this function should be considered a breaking change.
	fn from_seed(seed: u64) -> Random<Self>;
}

macro_rules! forward_seed_rng_impl {
	() => {
		/// Creates a new instance seeded securely from system entropy.
		///
		/// See the [`SeedRng`](SeedRng::new) trait for more information.
		#[inline]
		pub fn new() -> Random<Self> {
			SeedRng::new()
		}

		/// Creates a new PRNG seeded from another `Rng`.
		///
		/// See the [`SeedRng`](SeedRng::from_rng) trait for more information.
		#[inline]
		pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<Self> {
			SeedRng::from_rng(rand)
		}

		/// Creates a new PRNG using the given seed.
		///
		/// See the [`SeedRng`](SeedRng::from_seed) trait for more information.
		#[inline]
		pub fn from_seed(seed: u64) -> Random<Self> {
			SeedRng::from_seed(seed)
		}
	}
}

//----------------------------------------------------------------
// Random number generators

mod splitmix64;
pub use self::splitmix64::SplitMix64;

mod xoshiro256;
pub use self::xoshiro256::Xoshiro256;

mod wyrand;
pub use self::wyrand::Wyrand;

mod mock;
pub use self::mock::Mock;

cfg_if::cfg_if! {
	if #[cfg(feature = "std")] {
		mod read;
		pub use self::read::Read;
	}
}

mod chacha;
pub use self::chacha::{ChaCha, ChaCha8, ChaCha12, ChaCha20};

mod system;
pub use self::system::System;

mod entropy;
pub use self::entropy::{getentropy, getentropy_uninit};

//----------------------------------------------------------------

#[test]
fn test_trait_object() {
	// Ensure Rng is usable as a trait object
	fn test(rand: &mut Random<dyn Rng>) {
		let _: i32 = rand.next();
	}
	test(&mut crate::new());
	test(&mut crate::seeded(42));
	test(&mut crate::csprng());
}
