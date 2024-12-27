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
	// The current state of the ChaCha cipher
	state: [u32; NWORDS],
	// Consume the random words before producing more
	#[cfg_attr(feature = "serde", serde(default = "default_index", skip_serializing_if = "is_index_oob"))]
	index: u32,
	// The Rng produces 16 words per block
	#[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
	random: [u32; NWORDS],
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
		let mut state = [MaybeUninit::<u32>::uninit(); NWORDS];
		state[0].write(CONSTANT[0]);
		state[1].write(CONSTANT[1]);
		state[2].write(CONSTANT[2]);
		state[3].write(CONSTANT[3]);
		entropy::getentropy_uninit(&mut state[4..]);
		Random::wrap(ChaCha { state: unsafe { mem::transmute(state) }, index: NWORDS as u32, random: [0; NWORDS] })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master CSPRNG, and to allow forking of PRNGs.
	#[inline]
	pub fn from_rng<R: ?Sized + SecureRng>(rand: &mut Random<R>) -> Random<ChaCha<N>> {
		let mut state = [MaybeUninit::<u32>::uninit(); NWORDS];
		state[0].write(CONSTANT[0]);
		state[1].write(CONSTANT[1]);
		state[2].write(CONSTANT[2]);
		state[3].write(CONSTANT[3]);
		rand.fill_bytes_uninit(&mut state[4..]);
		Random::wrap(ChaCha { state: unsafe { mem::transmute(state) }, index: NWORDS as u32, random: [0; NWORDS] })
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
		let state = [
			CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
			low, high, low, high,
			low, high, low, high,
			1, 0, 0, 0,
		];
		Random::wrap(ChaCha { state, index: NWORDS as u32, random: [0; NWORDS] })
	}
}

impl<const N: usize> Rng for ChaCha<N> where Self: SecureRng {
	#[inline(never)]
	fn next_u32(&mut self) -> u32 {
		let mut index = self.index as usize;
		// Generate a new block if there are no more random words
		if index >= NWORDS {
			chacha_block(&mut self.state, &mut self.random, N);
			index = 0;
		}
		// Fetch a word from the random block
		let value = self.random[index];
		self.index = (index + 1) as u32;
		value
	}
	#[inline(never)]
	fn next_u64(&mut self) -> u64 {
		let mut index = self.index as usize;
		// Generate a new block if there are less than two random words
		if index >= NWORDS - 1 {
			chacha_block(&mut self.state, &mut self.random, N);
			index = 0;
		}
		// Fetch two words from the random block
		let low = self.random[index + 0] as u64;
		let high = self.random[index + 1] as u64;
		self.index = (index + 2) as u32;
		high << 32 | low
	}
	#[inline(never)]
	fn fill_bytes(&mut self, mut buf: &mut [MaybeUninit<u8>]) {
		// Fill directly from the generator
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = [0; NWORDS];
		while buf.len() >= BSIZE {
			chacha_block(&mut self.state, &mut tmp, N);
			unsafe { (buf.as_mut_ptr() as *mut [u32; NWORDS]).write_unaligned(tmp); }
			buf = &mut buf[BSIZE..];
		}
		// Fill the remaining bytes from the random block
		if buf.len() > 0 {
			loop {
				let index = usize::min(self.index as usize, NWORDS);
				let src = dataview::bytes(&self.random[index..]);
				let len = usize::min(src.len(), buf.len());
				unsafe { copy_bytes(src.as_ptr(), buf.as_mut_ptr(), len) };
				buf = &mut buf[len..];
				if buf.len() > 0 {
					chacha_block(&mut self.state, &mut self.random, N);
					self.index = 0;
				}
				else {
					self.index += (len + 3) as u32 / 4;
					break;
				}
			}
		}
	}
	#[inline]
	fn jump(&mut self) {
		increment_stream(&mut self.state);
		self.index = NWORDS as u32;
	}
}

#[inline(always)]
unsafe fn copy_bytes(mut src: *const u8, mut dst: *mut MaybeUninit<u8>, mut len: usize) {
	if len >= 32 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 32);
		src = src.add(32);
		dst = dst.add(32);
		len -= 32;
	}
	if len >= 16 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 16);
		src = src.add(16);
		dst = dst.add(16);
		len -= 16;
	}
	if len >= 8 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 8);
		src = src.add(8);
		dst = dst.add(8);
		len -= 8;
	}
	if len >= 4 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 4);
		src = src.add(4);
		dst = dst.add(4);
		len -= 4;
	}
	if len >= 2 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 2);
		src = src.add(2);
		dst = dst.add(2);
		len -= 2;
	}
	if len >= 1 {
		ptr::copy_nonoverlapping(src, dst as *mut u8, 1);
	}
}

//----------------------------------------------------------------

cfg_if::cfg_if! {
	if #[cfg(feature = "serde")] {
		fn is_default<T: Default + PartialEq>(value: &T) -> bool {
			*value == T::default()
		}
		fn is_index_oob(value: &u32) -> bool {
			*value >= NWORDS as u32
		}
		fn default_index() -> u32 {
			NWORDS as u32
		}
	}
}

//----------------------------------------------------------------
// ChaCha implementation details
// https://cr.yp.to/chacha/chacha-20080128.pdf
// http://loup-vaillant.fr/tutorials/chacha20-design

const CONSTANT: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];
const NWORDS: usize = 16;
const BSIZE: usize = 16 * 4;

#[inline]
fn increment_counter(state: &mut [u32; 16]) {
	let counter = (state[15] as u128) << 96 | (state[14] as u128) << 64 | (state[13] as u128) << 32 | (state[12] as u128) << 0;
	let counter = counter.wrapping_add(1);
	state[12] = (counter >> 0) as u32;
	state[13] = (counter >> 32) as u32;
	state[14] = (counter >> 64) as u32;
	state[15] = (counter >> 96) as u32;
}

#[inline]
fn increment_stream(state: &mut [u32; NWORDS]) {
	let stream = (state[15] as u64) << 32 | (state[14] as u64) << 0;
	let stream = stream.wrapping_add(1);
	state[14] = (stream & 0xffffffff) as u32;
	state[15] = (stream >> 32) as u32;
}

cfg_if::cfg_if! {
	if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))] {
		mod sse2;
		use self::sse2::block as chacha_block;
	}
	else {
		mod slp;
		use self::slp::block as chacha_block;
	}
}

#[test]
fn chacha20_selftest() {
	let mut state = [
		CONSTANT[0], CONSTANT[1], CONSTANT[2], CONSTANT[3],
		0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c,
		0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
		0x00000001, 0x09000000, 0x4a000000, 0x00000000,
	];
	let expected = [
		0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3,
		0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3,
		0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9,
		0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2,
	];
	let mut result = [0; NWORDS];
	chacha_block(&mut state, &mut result, 20);
	assert_eq!(expected, result);
}

#[test]
fn test_randomness() {
	let mut rand = ChaCha20::new();
	let mut words1 = [0u32; NWORDS];
	for i in 0..NWORDS {
		words1[i] = rand.next_u32();
	}
	let mut words2 = [0u32; NWORDS];
	for i in 0..NWORDS {
		words2[i] = rand.next_u32();
	}
	assert_ne!(words1, words2);
}

#[test]
fn test_fill_bytes() {
	let mut master = ChaCha20::new();
	master.next_u64();
	master.next_u64();
	master.next_u64();
	master.next_u32();
	let mut old = [0u8; 64];
	let mut buf = [0u8; 64];
	for i in 1..buf.len() {
		let mut rand = master.clone();
		rand.fill_bytes(&mut buf[..i]);
		assert_eq!(buf[..i - 1], old[..i - 1]);
		old = buf;
	}
}
