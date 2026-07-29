use super::*;
use std::cell::RefCell;


// Reseed the underlying generator after this many bytes have been generated.
const RESEED_THRESHOLD: usize = 64 * 1024;

std::thread_local! {
	static THREAD_RNG: RefCell<ThreadRngState> = RefCell::new(ThreadRngState::new());
}

struct ThreadRngState {
	rng: Random<ChaCha12Rng>,
	bytes_until_reseed: usize,
}

impl ThreadRngState {
	#[inline]
	fn new() -> ThreadRngState {
		ThreadRngState {
			rng: ChaCha12Rng::new(),
			bytes_until_reseed: RESEED_THRESHOLD,
		}
	}

	#[inline]
	fn reseed(&mut self) {
		self.rng = ChaCha12Rng::new();
		self.bytes_until_reseed = RESEED_THRESHOLD;
	}

	#[inline]
	fn generate<T>(&mut self, bytes: usize, f: impl FnOnce(&mut ChaCha12Rng) -> T) -> T {
		debug_assert!(bytes <= RESEED_THRESHOLD);
		if bytes > self.bytes_until_reseed {
			self.reseed();
		}
		let value = f(&mut self.rng);
		self.bytes_until_reseed -= bytes;
		value
	}

	#[inline]
	fn fill_bytes(&mut self, mut buf: &mut [MaybeUninit<u8>]) {
		while !buf.is_empty() {
			if self.bytes_until_reseed == 0 {
				self.reseed();
			}

			let len = usize::min(buf.len(), self.bytes_until_reseed);
			let (chunk, rest) = buf.split_at_mut(len);
			Rng::fill_bytes(&mut *self.rng, chunk);
			self.bytes_until_reseed -= len;
			buf = rest;
		}
	}
}

/// A stateless handle to the current thread's random number generator.
///
/// The generator is a lazily initialized [`ChaCha12Rng`] seeded from system entropy.
/// It is automatically reseeded before one seed would be used to generate more than 64 KiB of output.
/// All `ThreadRng` handles used by the same thread share its generator,
/// while handles used by different threads access independent generators.
///
/// # Forks
///
/// A process fork may duplicate the generator state. Call [`ThreadRng::reseed`]
/// immediately in the child process when using APIs such as `fork`.
///
/// # Panics
///
/// Initialization and reseeding panic if the system entropy source is unavailable.
/// Its methods may also panic when called reentrantly on the same thread.
#[derive(Clone, Debug)]
pub struct ThreadRng {
	_private: (),
}

impl ThreadRng {
	#[inline]
	pub(crate) const fn new() -> ThreadRng {
		ThreadRng { _private: () }
	}

	/// Immediately reseeds the current thread's generator from system entropy.
	///
	/// This discards the previous generator state and any buffered output.
	///
	/// # Panics
	///
	/// Panics if the system entropy source is unavailable.
	#[inline]
	pub fn reseed(&mut self) {
		THREAD_RNG.with_borrow_mut(ThreadRngState::reseed);
	}
}

impl Sealed for ThreadRng {}
impl SecureRng for ThreadRng {}

impl Rng for ThreadRng {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		with_rng(|rng| rng.generate(4, Rng::next_u32))
	}

	#[inline]
	fn next_u64(&mut self) -> u64 {
		with_rng(|rng| rng.generate(8, Rng::next_u64))
	}

	#[inline]
	fn next_f32(&mut self) -> f32 {
		with_rng(|rng| rng.generate(4, Rng::next_f32))
	}

	#[inline]
	fn next_f64(&mut self) -> f64 {
		with_rng(|rng| rng.generate(8, Rng::next_f64))
	}

	#[inline]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		with_rng(|rng| rng.fill_bytes(buf));
	}
}

#[inline]
fn with_rng<T>(f: impl FnOnce(&mut ThreadRngState) -> T) -> T {
	THREAD_RNG.with_borrow_mut(|rng| f(rng))
}

const _: [(); 1] = [(); (core::mem::size_of::<ThreadRng>() == 0) as usize];

#[cfg(test)]
mod tests;
