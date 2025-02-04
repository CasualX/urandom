/*!
Random number generators.

Pseudorandom number generators
-------------------------------

These generators implement fast PRNG which are suitable for normal use in non-cryptography applications.

* [`Xoshiro256`] Rng:

  See the excellent [PRNG shootout](http://prng.di.unimi.it/) article.

* [`SplitMix64`] Rng:

  Fast RNG, with 64 bits of state, that can be used to initialize the state of other generators.

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

mod block;
use self::block::{BlockRng, BlockRngImpl};

//----------------------------------------------------------------

#[cfg(test)]
mod tests;
