use core::iter;
use super::*;

/// Random number generator mock.
///
/// Produces randomness directly from the given iterator and panics when it runs out of items.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Mock<I>(pub I);

impl<'a> Mock<iter::Copied<slice::Iter<'a, u64>>> {
	/// Produces the values from the input slice as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::Mock;
	///
	/// let mut rand = Mock::slice(&[1, 2, 13, 42]);
	///
	/// assert_eq!(rand.next_u64(), 1);
	/// assert_eq!(rand.next_u64(), 2);
	/// assert_eq!(rand.next_u64(), 13);
	/// assert_eq!(rand.next_u64(), 42);
	///
	/// // Any further calls to the Mock will panic unless the underlying iterator is unbounded.
	/// ```
	pub fn slice(slice: &'a [u64]) -> Random<Self> {
		Random::from(Mock(slice.iter().copied()))
	}
}
impl Mock<iter::Repeat<u64>> {
	/// Produces the same random number repeatedly as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::Mock;
	///
	/// let mut rand = Mock::repeat(42);
	///
	/// assert_eq!(rand.next_u64(), 42);
	/// assert_eq!(rand.next_u64(), 42);
	/// assert_eq!(rand.next_u64(), 42);
	/// ```
	pub fn repeat(value: u64) -> Random<Self> {
		Random::from(Mock(iter::repeat(value)))
	}
}

impl<I> Rng for Mock<I> where I: Iterator<Item = u64> {
	fn next_u32(&mut self) -> u32 {
		self.0.next().unwrap() as u32
	}
	fn next_u64(&mut self) -> u64 {
		self.0.next().unwrap()
	}
	fn fill_bytes(&mut self, _buf: &mut [MaybeUninit<u8>]) {
		unimplemented!()
	}
	fn jump(&mut self) {
		unimplemented!()
	}
}
