use core::{iter, slice};
use crate::{Random, Rng};

/// Random number generator mock.
///
/// Produces randomness directly from the given iterator and panics when it runs out of items.
#[derive(Clone, Debug)]
pub struct MockRng<I>(pub I);

impl<'a> MockRng<iter::Copied<slice::Iter<'a, u64>>> {
	/// Produces the values from the input slice as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::MockRng;
	///
	/// let mut rng = MockRng::slice(&[1, 2, 13, 42]);
	///
	/// assert_eq!(rng.next_u64(), 1);
	/// assert_eq!(rng.next_u64(), 2);
	/// assert_eq!(rng.next_u64(), 13);
	/// assert_eq!(rng.next_u64(), 42);
	///
	/// // Any further calls to the MockRng will panic unless the underlying iterator is unbounded.
	/// ```
	pub fn slice(slice: &'a [u64]) -> Random<Self> {
		Random(MockRng(slice.iter().copied()))
	}
}
impl MockRng<iter::Repeat<u64>> {
	/// Produces the same random number repeatedly as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::MockRng;
	///
	/// let mut rng = MockRng::repeat(42);
	///
	/// assert_eq!(rng.next_u64(), 42);
	/// assert_eq!(rng.next_u64(), 42);
	/// assert_eq!(rng.next_u64(), 42);
	/// ```
	pub fn repeat(value: u64) -> Random<Self> {
		Random(MockRng(iter::repeat(value)))
	}
}

impl<I> Rng for MockRng<I> where I: Iterator<Item = u64> {
	fn next_u32(&mut self) -> u32 {
		self.0.next().unwrap() as u32
	}
	fn next_u64(&mut self) -> u64 {
		self.0.next().unwrap()
	}
	fn fill_u32(&mut self, buffer: &mut [u32]) {
		for slot in buffer {
			*slot = self.next_u32();
		}
	}
	fn fill_u64(&mut self, buffer: &mut [u64]) {
		for slot in buffer {
			*slot = self.next_u64();
		}
	}
	fn fill_bytes(&mut self, _buffer: &mut [u8]) {
		unimplemented!()
	}
	fn jump(&mut self) {
		// This method is intentionally left blank.
	}
}
