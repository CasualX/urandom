/*!
Random number generators.

Reproducibility
---------------

Deterministic generators are reproducible from the same seed only when given
the same sequence of calls with the same arguments. Different [`Rng`] methods
are not interchangeable views of a single random stream.

Pseudorandom number generators
-------------------------------

These are fast pseudorandom number generators suitable for ordinary, non-cryptographic applications.

* [`Xoshiro256Rng`]:

  See the [PRNG shootout](http://prng.di.unimi.it/) for background and analysis.

* [`SplitMix64Rng`]:

  Fast RNG, with 64 bits of state, that can be used to initialize the state of other generators.

Cryptographically secure generators
-----------------------------------

These generators are suitable for cryptographic applications.

* [`ChaCha8Rng`], [`ChaCha12Rng`], [`ChaCha20Rng`]:

  Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.

  The [`csprng`](crate::csprng) constructor uses [`ChaCha12Rng`]. This choice is stable across compatible versions of this crate.
  See this [`rand` discussion](https://github.com/rust-random/rand/issues/932) for background on the choice of round count.

* [`SystemRng`]:

  Reads randomness directly from the system entropy source.

  For performance, this generator fetches entropy in blocks of `N` 32-bit words.
  Larger values of `N` reduce how often the system entropy source must be called.

Other generators
----------------

* [`MockRng`]:

  A deterministic test generator backed by an iterator. It panics when the iterator runs out of items.

*/

#![cfg_attr(feature = "std", doc = r#"
* [`ReadRng`]:

  Reads bytes from any source implementing [`std::io::Read`], such as a file or device.
"#)]

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
///
/// Implementations that promise reproducibility must return the same output
/// from the same initial state for an identical sequence of calls and arguments.
/// Different methods do not need to consume equivalent parts of one random stream.
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
	/// Generators that do not support jumping panic.
	/// See the concrete generator's documentation for its exact behavior.
	fn jump(&mut self);
}

/// Marker trait for random number generators suitable for cryptographic use.
///
/// This marker describes the algorithm, not how a particular instance was initialized.
/// A manually seeded generator cannot provide more unpredictability than its seed;
/// cryptographic use requires a secret seed with sufficient entropy.
pub trait SecureRng: Rng {}

//----------------------------------------------------------------
// Random number generators

mod splitmix64;
pub use self::splitmix64::SplitMix64Rng;

mod xoshiro256;
pub use self::xoshiro256::Xoshiro256Rng;

// mod wyrand;
// pub use self::wyrand::WyrandRng;

mod mock;
pub use self::mock::MockRng;

cfg_if::cfg_if! {
	if #[cfg(feature = "std")] {
		mod read;
		pub use self::read::ReadRng;
	}
}

mod chacha;
pub use self::chacha::{ChaChaRng, ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};

mod system;
pub use self::system::SystemRng;

mod entropy;
pub use self::entropy::{getentropy, getentropy_uninit};

mod block;
use self::block::{BlockRng, BlockRngImpl};

//----------------------------------------------------------------

#[cfg(test)]
mod tests;
