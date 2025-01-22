use super::*;


/// [`ChaCha`] with 8 rounds.
pub type ChaCha8 = ChaCha<8>;
impl SecureRng for ChaCha<8> {}

/// [`ChaCha`] with 12 rounds.
pub type ChaCha12 = ChaCha<12>;
impl SecureRng for ChaCha<12> {}

/// [`ChaCha`] with 20 rounds.
pub type ChaCha20 = ChaCha<20>;
impl SecureRng for ChaCha<20> {}


/// Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChaCha<const N: usize> {
	#[cfg_attr(feature = "serde", serde(flatten))]
	inner: BlockRngImpl<ChaChaState<N>>,
}

impl<const N: usize> ChaCha<N> where Self: SecureRng {
	/// Creates a new instance seeded securely from system entropy.
	///
	/// This method is the recommended way to construct PRNGs since it is convenient and secure.
	///
	/// # Panics
	///
	/// If [`getentropy`] is unable to provide secure entropy this method will panic.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::ChaCha12::new();
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn new() -> Random<ChaCha<N>> {
		let state = util::getrandom();
		let inner = BlockRngImpl::new(state);
		Random::wrap(ChaCha { inner })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master CSPRNG, and to allow forking of PRNGs.
	#[inline]
	pub fn from_rng<R: ?Sized + SecureRng>(rand: &mut Random<R>) -> Random<ChaCha<N>> {
		let state = rand.random_bytes();
		let inner = BlockRngImpl::new(state);
		Random::wrap(ChaCha { inner })
	}

	/// Creates a new instance using the given seed.
	///
	/// This **is not suitable for cryptography**, as should be clear given that the input size is only 64 bits.
	///
	/// Implementations are required to be reproducible given the same seed.
	/// _Changing_ the implementation of this function should be considered a breaking change.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::ChaCha12::from_seed(42);
	/// let value: i32 = rand.next();
	/// assert_eq!(value, 631540493);
	/// ```
	#[inline]
	pub fn from_seed(seed: u64) -> Random<ChaCha<N>> {
		let low = (seed & 0xffffffff) as u32;
		let high = (seed >> 32) as u32;
		let state = ChaChaState::new([low, high, low, high, low, high, low, high], 1, 0);
		let inner = BlockRngImpl::new(state);
		Random::wrap(ChaCha { inner })
	}
}

impl<const N: usize> Rng for ChaCha<N> where Self: SecureRng {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		self.inner.next_u32()
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		self.inner.next_u64()
	}
	#[inline]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		self.inner.fill_bytes(buf);
	}
	#[inline]
	fn jump(&mut self) {
		self.inner.jump();
	}
}


//----------------------------------------------------------------
// ChaCha implementation details
// https://cr.yp.to/chacha/chacha-20080128.pdf
// http://loup-vaillant.fr/tutorials/chacha20-design

use core::fmt;

cfg_if::cfg_if! {
	if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))] {
		mod avx2;
		use self::avx2::block as chacha_block;
	}
	else if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))] {
		mod sse2;
		use self::sse2::block as chacha_block;
	}
	else {
		mod slp;
		use self::slp::block as chacha_block;
	}
}

const CN: usize = 4; // Concurrent ChaCha instances
const CONSTANT: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

#[derive(Clone)]
#[repr(C)]
pub struct ChaChaState<const N: usize> {
	seed: [u32; 8],
	counter: [u32; 2],
	stream: [u32; 2],
}

unsafe impl<const N: usize> dataview::Pod for ChaChaState<N> {}

impl<const N: usize> ChaChaState<N> {
	#[inline]
	pub fn new(seed: [u32; 8], counter: u64, stream: u64) -> ChaChaState<N> {
		ChaChaState {
			seed,
			counter: [counter as u32, (counter >> 32) as u32],
			stream: [stream as u32, (stream >> 32) as u32],
		}
	}
	#[inline]
	pub fn get_state(&self) -> [[u32; 4]; 4] {
		[
			CONSTANT,
			[self.seed[0], self.seed[1], self.seed[2], self.seed[3]],
			[self.seed[4], self.seed[5], self.seed[6], self.seed[7]],
			[self.counter[0], self.counter[1], self.stream[0], self.stream[1]],
		]
	}
	#[inline]
	pub fn get_counter(&self) -> u64 {
		(self.counter[1] as u64) << 32 | self.counter[0] as u64
	}
	#[inline]
	pub fn set_counter(&mut self, counter: u64) {
		self.counter[0] = counter as u32;
		self.counter[1] = (counter >> 32) as u32;
	}
	#[inline]
	pub fn add_counter(&self, counter: u64) -> ChaChaState<N> {
		let mut this = self.clone();
		this.set_counter(self.get_counter().wrapping_add(counter));
		this
	}
	#[inline]
	pub fn get_stream(&self) -> u64 {
		(self.stream[1] as u64) << 32 | self.stream[0] as u64
	}
	#[inline]
	pub fn set_stream(&mut self, stream: u64) {
		self.stream[0] = stream as u32;
		self.stream[1] = (stream >> 32) as u32;
	}
}

impl<const N: usize> BlockRng for ChaChaState<N> {
	type Output = [[u32; 16]; CN];

	#[inline]
	fn generate(&mut self, random: &mut Self::Output) {
		chacha_block(self, random);
	}

	#[inline]
	fn jump(&mut self) {
		self.set_stream(self.get_stream().wrapping_add(1));
	}
}

impl<const N: usize> fmt::Debug for ChaChaState<N> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ChaChaCore")
			.field("seed", &format_args!("{:x?}", self.seed))
			.field("counter", &self.get_counter())
			.field("stream", &self.get_stream())
			.finish()
	}
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for ChaChaState<N> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		[
			self.seed[0], self.seed[1], self.seed[2], self.seed[3],
			self.seed[4], self.seed[5], self.seed[6], self.seed[7],
			self.counter[0], self.counter[1], self.stream[0], self.stream[1],
		].serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for ChaChaState<N> {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let values = <[u32; 12]>::deserialize(deserializer)?;
		Ok(ChaChaState {
			seed: [values[0], values[1], values[2], values[3], values[4], values[5], values[6], values[7]],
			counter: [values[8], values[9]],
			stream: [values[10], values[11]],
		})
	}
}

#[cfg(test)]
mod tests;
