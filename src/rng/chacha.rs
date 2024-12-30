use super::*;

mod inner;
use self::inner::ChaChaCore;


/// [`ChaCha`] with 8 rounds.
pub type ChaCha8 = ChaCha<8>;
impl SecureRng for ChaCha<8> {}

/// [`ChaCha`] with 12 rounds.
pub type ChaCha12 = ChaCha<12>;
impl SecureRng for ChaCha<12> {}

/// [`ChaCha`] with 20 rounds.
pub type ChaCha20 = ChaCha<20>;
impl SecureRng for ChaCha<20> {}


const CN: usize = 4; // Concurrent ChaCha instances
const INDEX: u32 = 16 * 4;
const RANDOM: [[u32; 16]; CN] = [[0; 16]; CN]; // Default random blocks


/// Daniel J. Bernstein's ChaCha adapted as a deterministic random number generator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChaCha<const N: usize> {
	// The current state of the ChaCha cipher
	state: ChaChaCore,
	// Consume the random words before producing more
	#[cfg_attr(feature = "serde", serde(default = "default_index", skip_serializing_if = "is_index_oob"))]
	index: u32,
	// The Rng produces 16 words per block
	#[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
	random: [[u32; 16]; CN],
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
		Random::wrap(ChaCha { state, index: INDEX, random: RANDOM })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master CSPRNG, and to allow forking of PRNGs.
	#[inline]
	pub fn from_rng<R: ?Sized + SecureRng>(rand: &mut Random<R>) -> Random<ChaCha<N>> {
		let state = rand.random_bytes();
		Random::wrap(ChaCha { state, index: INDEX, random: RANDOM })
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
		let state = ChaChaCore::new([low, high, low, high, low, high, low, high], 1, 0);
		Random::wrap(ChaCha { state, index: INDEX, random: RANDOM })
	}
}

#[inline]
fn read_u32(random: &[[u32; 16]; CN], index: usize) -> u32 {
	let index = index & 0x3f;
	let block = index / 16;
	let word = index % 16;
	random[block][word]
}

#[inline]
fn flatten_random(random: &[[u32; 16]; CN]) -> &[u32; 16 * CN] {
	unsafe { mem::transmute(random) }
}

impl<const N: usize> Rng for ChaCha<N> where Self: SecureRng {
	#[inline(never)]
	fn next_u32(&mut self) -> u32 {
		let mut index = self.index as usize;
		// Generate a new block if there are no more random words
		if index > INDEX as usize - 1 {
			chacha_block(&mut self.state, &mut self.random, N);
			index = 0;
		}
		// Fetch a word from the random block
		let value = read_u32(&self.random, index);
		self.index = (index + 1) as u32;
		value
	}
	#[inline(never)]
	fn next_u64(&mut self) -> u64 {
		let mut index = self.index as usize;
		// Generate a new block if there are less than two random words
		if index > INDEX as usize - 2 {
			chacha_block(&mut self.state, &mut self.random, N);
			index = 0;
		}
		// Fetch two words from the random block
		let low = read_u32(&self.random, index + 0) as u64;
		let high = read_u32(&self.random, index + 1) as u64;
		self.index = (index + 2) as u32;
		high << 32 | low
	}
	#[inline(never)]
	fn fill_bytes(&mut self, mut buf: &mut [MaybeUninit<u8>]) {
		// Fill directly from the generator
		// Use a temporary block buffer due to potential alignment issues
		let mut tmp = [[0; 16]; CN];
		while buf.len() >= mem::size_of_val(&tmp) {
			chacha_block(&mut self.state, &mut tmp, N);
			unsafe { (buf.as_mut_ptr() as *mut [[u32; 16]; CN]).write_unaligned(tmp); }
			buf = &mut buf[mem::size_of_val(&tmp)..];
		}
		// Fill the remaining bytes from the random block
		if buf.len() > 0 {
			loop {
				let random = flatten_random(&self.random);
				let start = u32::min(self.index, INDEX) as usize;
				let src = dataview::bytes(&random[start..]);
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
		self.state.set_stream(self.state.get_stream().wrapping_add(1));
		self.index = INDEX;
	}
}

//----------------------------------------------------------------

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

cfg_if::cfg_if! {
	if #[cfg(feature = "serde")] {
		fn is_default<T: Default + PartialEq>(value: &T) -> bool {
			*value == T::default()
		}
		fn is_index_oob(value: &u32) -> bool {
			*value >= INDEX
		}
		fn default_index() -> u32 {
			INDEX
		}
	}
}

//----------------------------------------------------------------
// ChaCha implementation details
// https://cr.yp.to/chacha/chacha-20080128.pdf
// http://loup-vaillant.fr/tutorials/chacha20-design

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

#[cfg(test)]
mod tests;
