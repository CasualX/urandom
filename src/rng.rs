//! Random number generators.
//!
//! Reproducibility guarantee
//! -------------------------
//!
//! The random stream of each deterministic generator in this module is part of its stable API.
//! Across SemVer-compatible releases, a concrete generator initialized from the same explicit seed or state
//! produces identical output on supported targets when given the same sequence of [`Rng`] and [`JumpRng`] calls and arguments.
//! The generator algorithm, seed expansion, and behavior of those calls will not be changed, including for performance improvements.
//!
//! The exact call sequence is part of the input to this guarantee. For example, one [`Rng::next_u64`] call
//! is not interchangeable with two [`Rng::next_u32`] calls, even if both request 64 bits in total.
//!
//! With the `serde` feature, the guarantee also covers serialized generator state. State written by one release
//! remains readable by SemVer-compatible releases, and restoring it continues the same stream at the saved position.
//!
#![cfg_attr(feature = "getrandom", doc = "This guarantee also applies to the root-level [`crate::new`] and [`crate::seeded`] constructors: their generator choice and initialization behavior remain stable across SemVer-compatible releases.")]
#![cfg_attr(not(feature = "getrandom"), doc = "This guarantee also applies to the root-level [`crate::seeded`] constructor: its generator choice and initialization behavior remain stable across SemVer-compatible releases.")]
//!
//! Pseudorandom number generators
//! -------------------------------
//!
//! These are fast pseudorandom number generators suitable for ordinary, non-cryptographic applications.
//!
//! * [`Xoshiro256Rng`]:
//!
//!   See the [PRNG shootout](http://prng.di.unimi.it/) for background and analysis.
//!
//! Cryptographically secure generators
//! -----------------------------------
//!
//! These generators are suitable for cryptographic applications.
//!
//! * [`ChaCha8Rng`], [`ChaCha12Rng`], [`ChaCha20Rng`]:
//!
//!   Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.
//!
#![cfg_attr(feature = "getrandom", doc = "* [`SystemRng`]:")]
#![cfg_attr(not(feature = "getrandom"), doc = "* `SystemRng` (requires the `getrandom` feature):")]
//!
//!   Reads randomness directly from the system entropy source.
//!
//!   For performance, this generator fetches entropy in blocks of `N` 32-bit words.
//!   Larger values of `N` reduce how often the system entropy source must be called.
//!
//! Other generators
//! ----------------
//!
//! * [`MockRng`]:
//!
//!   A deterministic test generator backed by an iterator. It panics when the iterator runs out of items.

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

mod pod;
use pod::Pod;

mod sealed;
use sealed::Sealed;

/// Random number generator interface.
///
/// This trait is sealed and cannot be implemented outside this crate.
///
/// Deterministic implementations provided by this module follow the module-level
/// [reproducibility guarantee](crate::rng#reproducibility-guarantee).
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
}

/// Random number generator interface for generators that support jumping ahead.
pub trait JumpRng: Rng {
	/// Advances the generator's state by a large, implementation-defined distance.
	fn jump(&mut self);

	/// Consumes this generator and returns two independently reseeded descendants.
	fn fork(self) -> (Self, Self) where Self: Sized;
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
#[doc(hidden)] // Not intended to be used publicly, but unfortunately cannot be fully removed at this time
pub use self::splitmix64::SplitMix64Rng;

mod xoshiro256;
pub use self::xoshiro256::Xoshiro256Rng;

// mod wyrand;
// pub use self::wyrand::WyrandRng;

mod mock;
pub use self::mock::MockRng;

cfg_select! {
	feature = "std" => {
		mod read;
		pub use self::read::ReadRng;
	}
	_ => {}
}

mod chacha;
pub use self::chacha::{ChaChaRng, ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};

#[cfg(feature = "getrandom")]
mod system;
#[cfg(feature = "getrandom")]
pub use self::system::SystemRng;

#[cfg(feature = "getrandom")]
mod entropy;
#[cfg(feature = "getrandom")]
pub use self::entropy::{getentropy, getentropy_uninit};

mod block;
use self::block::{BlockRng, BlockRngImpl};

//----------------------------------------------------------------

#[cfg(test)]
mod tests;
