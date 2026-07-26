use core::iter;
use super::*;

/// Random number generator mock.
///
/// Produces randomness directly from the given iterator and panics when it runs out of items.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct MockRng<I>(pub I);

impl<'a> MockRng<iter::Copied<slice::Iter<'a, u64>>> {
	/// Produces the values from the input slice as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::MockRng;
	///
	/// let mut rand = MockRng::slice(&[1, 2, 13, 42]);
	///
	/// assert_eq!(rand.random::<u64>(), 1);
	/// assert_eq!(rand.random::<u64>(), 2);
	/// assert_eq!(rand.random::<u64>(), 13);
	/// assert_eq!(rand.random::<u64>(), 42);
	///
	/// // Any further calls to the MockRng will panic unless the underlying iterator is unbounded.
	/// ```
	#[inline]
	pub fn slice(slice: &'a [u64]) -> Random<Self> {
		Random::wrap(MockRng(slice.iter().copied()))
	}
}
impl MockRng<iter::Repeat<u64>> {
	/// Produces the same random number repeatedly as the underlying random number generator.
	///
	/// ```
	/// use urandom::rng::MockRng;
	///
	/// let mut rand = MockRng::repeat(42);
	///
	/// assert_eq!(rand.random::<u64>(), 42);
	/// assert_eq!(rand.random::<u64>(), 42);
	/// assert_eq!(rand.random::<u64>(), 42);
	/// ```
	#[inline]
	pub fn repeat(value: u64) -> Random<Self> {
		Random::wrap(MockRng(iter::repeat(value)))
	}
}

impl<I> Sealed for MockRng<I> {}

impl<I> Rng for MockRng<I> where I: Iterator<Item = u64> {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		self.0.next().unwrap() as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		self.0.next().unwrap()
	}
	#[inline]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		util::rng_fill_bytes(self, buf);
	}
	#[inline]
	fn jump(&mut self) {
		unimplemented!()
	}
}
