/*!
Random number generators.

Pseudorandom number generators
-------------------------------

These are fast pseudorandom number generators suitable for ordinary, non-cryptographic applications.

* [`Xoshiro256`]:

  See the [PRNG shootout](http://prng.di.unimi.it/) for background and analysis.

* [`SplitMix64`]:

  Fast RNG, with 64 bits of state, that can be used to initialize the state of other generators.

* [`Wyrand`]:

  Tiny and very fast pseudorandom number generator based on [rapidhash](https://github.com/Nicoshev/rapidhash).

Cryptographically secure generators
-----------------------------------

These generators are suitable for cryptographic applications.

* [`ChaCha8`], [`ChaCha12`], [`ChaCha20`]:

  Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.

  The [`csprng`](crate::csprng) constructor uses [`ChaCha12`]. This choice is stable across compatible versions of this crate.
  See this [`rand` discussion](https://github.com/rust-random/rand/issues/932) for background on the choice of round count.

* [`System`]:

  Reads randomness directly from the system entropy source.

  For performance, this generator fetches entropy in blocks of `N` 32-bit words.
  Larger values of `N` reduce how often the system entropy source must be called.

Other generators
----------------

* [`Mock`]:

  A deterministic test generator backed by an iterator. It panics when the iterator runs out of items.

* [`Read`]:

  Reads bytes from any source implementing [`std::io::Read`], such as a file or device.

*/

// RNG implementations may use unsafe code where required
#![allow(unsafe_code)]

use core::{mem, ptr, slice};
use core::mem::MaybeUninit;

use crate::Random;

pub(crate) mod util;

mod sealed;
use sealed::Sealed;

/// Random number generator interface.
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Rng: Sealed {
	/// Returns the next `u32` in the sequence.
	fn next_u32(&mut self) -> u32;

	/// Returns the next `u64` in the sequence.
	fn next_u64(&mut self) -> u64;

	/// Returns a uniform random `f32` in the half-open interval `[1.0, 2.0)`.
	///
	/// Because only 23 bits are needed to construct an `f32` in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// The default implementation uses bits from [`Rng::next_u32`].
	#[inline]
	fn next_f32(&mut self) -> f32 {
		util::rng_f32(self.next_u32())
	}

	/// Returns a uniform random `f64` in the half-open interval `[1.0, 2.0)`.
	///
	/// Because only 52 bits are needed to construct an `f64` in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// The default implementation uses bits from [`Rng::next_u64`].
	#[inline]
	fn next_f64(&mut self) -> f64 {
		util::rng_f64(self.next_u64())
	}

	/// Fills every element of `buf` with random data.
	///
	/// On return, every element of `buf` is initialized.
	///
	/// Implementations must produce identical output on little-endian and big-endian targets.
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]);

	/// Advances the generator's state by a large, implementation-defined distance.
	///
	/// For deterministic pseudorandom generators, this may be used to derive separate streams for parallel computation.
	///
	/// Some generators, such as external entropy sources or test generators, may implement this as a no-op or panic.
	/// See the concrete generator's documentation for its exact behavior.
	fn jump(&mut self);
}

/// Marker trait for random number generators suitable for cryptographic use.
pub trait SecureRng: Rng {}

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
