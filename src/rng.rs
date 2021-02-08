/*!
Random number generators.

Pseudorandom number generators
-------------------------------

These generators implement fast PRNG which are suitable for normal use in non-cryptography applications.

* [`SplitMix64`](SplitMix64) Rng:

  Simple 64-bit state generator uses wrapping addition, wrapping multiplication, XOR and shifts.

* [`Xoshiro256`](Xoshiro256) Rng:

  Kindly taken from [Sebastiano Vigna](http://vigna.di.unimi.it/)'s excellent [PRNG shootout](http://prng.di.unimi.it/) article.

Cryptographically secure generators
-----------------------------------

These generators implement suitable CSPRNG implementations without calling out to the system or hardware directly.

* [`ChaCha20`](ChaCha20) Rng:

  Daniel J. Bernstein's ChaCha20 adapted as a deterministic random number generator.

*/

#![allow(unsafe_code)]

use crate::Random;

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
	fn next_f32(&mut self) -> f32 {
		crate::impls::rng_f32(self.next_u32())
	}

	/// Returns a uniform random `f64` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 52 bits are necessary to construct a random double in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// The default implementation simply gets its random bits from `next_u64`.
	fn next_f64(&mut self) -> f64 {
		crate::impls::rng_f64(self.next_u64())
	}

	/// Fills the next `u32` elements in the sequence.
	///
	/// Implementations are not required to implement this method with `next_u32`.
	/// This may produce distinct values compared to filling naively with `next_u32`.
	///
	/// Implementations are required to produce the same result regardless of endianness.
	fn fill_u32(&mut self, buffer: &mut [u32]);

	/// Fills the next `u64` elements in the sequence.
	fn fill_u64(&mut self, buffer: &mut [u64]);

	/// Fills the byte slice with uniform random bytes.
	///
	/// Implementations are required to produce the same result regardless of endianness.
	fn fill_bytes(&mut self, buffer: &mut [u8]);

	/// Advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	fn jump(&mut self);
}

/// Constructors for deterministic random number generators.
pub trait SeedRng: Sized {
	/// Creates a new instance seeded securely from system entropy.
	///
	/// This method is the recommended way to construct PRNGs since it is convenient and secure.
	///
	/// # Panics
	///
	/// If [`getentropy`](getentropy) is unable to provide secure entropy this method will panic.
	fn new() -> Random<Self>;

	/// Creates a new PRNG seeded from another `Rng`.
	///
	/// This may be useful when needing to rapidly seed many PRNGs from a master PRNG, and to allow forking of PRNGs.
	///
	/// The master PRNG should use a sufficiently different algorithm from the child PRNG (ideally a CSPRNG) to avoid correlations between the child PRNGs.
	fn from_rng<R: Rng + ?Sized>(rng: &mut Random<R>) -> Random<Self>;

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
	($ty:ty) => {
		impl $ty {
			/// Creates a new instance seeded securely from system entropy.
			///
			/// See the [`SeedRng`](SeedRng::new) trait for more information.
			#[inline]
			pub fn new() -> Random<$ty> {
				SeedRng::new()
			}
			/// Creates a new PRNG seeded from another `Rng`.
			///
			/// See the [`SeedRng`](SeedRng::from_rng) trait for more information.
			#[inline]
			pub fn from_rng<R: Rng + ?Sized>(rng: &mut Random<R>) -> Random<$ty> {
				SeedRng::from_rng(rng)
			}
			/// Creates a new PRNG using the given seed.
			///
			/// See the [`SeedRng`](SeedRng::from_seed) trait for more information.
			#[inline]
			pub fn from_seed(seed: u64) -> Random<$ty> {
				SeedRng::from_seed(seed)
			}
		}
	}
}

//----------------------------------------------------------------
// Random number generators

mod splitmix64;
pub use self::splitmix64::SplitMix64;

mod xoshiro256;
pub use self::xoshiro256::Xoshiro256;

mod mock;
pub use self::mock::MockRng;

mod chacha20;
pub use self::chacha20::ChaCha20;

mod entropy;
pub use self::entropy::getentropy;

//----------------------------------------------------------------

#[test]
fn test_trait_object() {
	// Ensure Rng is usable as a trait object
	let mut rng = crate::new();
	fn test(rng: &mut Random<dyn Rng>) {
		let _: i32 = rng.next();
	}
	test(&mut rng);
}
