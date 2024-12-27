use super::*;

impl<const N: usize> SecureRng for System<N> {}

/// Randomness directly from the system entropy source.
///
/// For performance reasons this generator fetches entropy in blocks of `N` 32-bit words.
/// The bigger `N` is, the less often the system entropy source is called.
pub struct System<const N: usize> {
	index: u32,
	random: [u32; N],
}

impl<const N: usize> System<N> {
	/// Creates a new instance.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::System::<31>::new();
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn new() -> Random<Self> {
		Random::wrap(Self {
			index: !0,
			random: [0; N],
		})
	}
}

impl<const N: usize> Rng for System<N> {
	fn next_u32(&mut self) -> u32 {
		let mut index = self.index as usize;
		// Generate a new block if there are no more random words
		if index >= N {
			getentropy(&mut self.random);
			index = 0;
		}
		// Fetch a word from the random block
		let value = self.random[index];
		self.index = (index + 1) as u32;
		value
	}
	fn next_u64(&mut self) -> u64 {
		let mut index = self.index as usize;
		// Generate a new block if there are less than two random words
		if index >= N - 1 {
			getentropy(&mut self.random);
			index = 0;
		}
		// Fetch two words from the random block
		let low = self.random[index + 0] as u64;
		let high = self.random[index + 1] as u64;
		self.index = (index + 2) as u32;
		high << 32 | low
	}
	#[inline]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		getentropy_uninit(buf);
	}
	#[inline]
	fn jump(&mut self) {
		self.index = !0;
	}
}
