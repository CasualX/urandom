use super::*;

/// The xoshiro256 random number generator[^1].
///
/// It has excellent (sub-ns) speed, a state (256 bits) that is large enough for any parallel application, and it passes all tests we are aware of.
///
/// [^1]: David Blackman and Sebastiano Vigna, 2021. [*Scrambled Llinear Pseudorandom Number Generators*](https://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Xoshiro256 {
	state: [u64; 4],
}

impl Xoshiro256 {
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
	/// let mut rand = urandom::rng::Xoshiro256::new();
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn new() -> Random<Xoshiro256> {
		let state = util::getrandom();
		Random::wrap(Xoshiro256 { state })
	}

	/// Creates a new instance seeded from another generator.
	///
	/// This may be useful when needing to rapidly seed many instances from a master PRNG, and to allow forking of PRNGs.
	///
	/// The master PRNG should use a sufficiently different algorithm from the child PRNG (ideally a CSPRNG) to avoid correlations between the child PRNGs.
	///
	/// # Examples
	///
	/// ```
	/// let mut master = urandom::rng::SplitMix64::new();
	/// let mut rand = urandom::rng::Xoshiro256::from_rng(&mut master);
	/// let value: i32 = rand.next();
	/// ```
	#[inline]
	pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<Xoshiro256> {
		let state = rand.random_bytes();
		Random::wrap(Xoshiro256 { state })
	}

	/// Creates a new instance using the given seed.
	///
	/// Implementations are required to be reproducible given the same seed.
	/// _Changing_ the implementation of this function should be considered a breaking change.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::Xoshiro256::from_seed(42);
	/// let value = rand.next_u32();
	/// assert_eq!(value, 368317477);
	/// ```
	pub fn from_seed(seed: u64) -> Random<Xoshiro256> {
		let mut master = SplitMix64::from_seed(seed);
		let state = [master.next_u64(), master.next_u64(), master.next_u64(), master.next_u64()];
		Random::wrap(Xoshiro256 { state })
	}
}

impl Rng for Xoshiro256 {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		(next_plus(&mut self.state) >> 32) as u32
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		next_plusplus(&mut self.state)
	}
	#[inline]
	fn next_f32(&mut self) -> f32 {
		util::rng_f32((next_plus(&mut self.state) >> 32) as u32)
	}
	#[inline]
	fn next_f64(&mut self) -> f64 {
		util::rng_f64(next_plus(&mut self.state))
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		let mut rng = self.clone();
		util::rng_fill_bytes(&mut rng, buf);
		*self = rng;
	}
	#[inline(never)]
	fn jump(&mut self) {
		jump(&mut self.state)
	}
}

//----------------------------------------------------------------
// Xoshiro256 implementation details

#[inline]
fn advance(s: &mut [u64; 4]) {
	let t = s[1] << 17;

	s[2] ^= s[0];
	s[3] ^= s[1];
	s[1] ^= s[2];
	s[0] ^= s[3];

	s[2] ^= t;

	s[3] = s[3].rotate_left(45);
}
#[inline]
fn next_plusplus(s: &mut [u64; 4]) -> u64 {
	let result = u64::wrapping_add(u64::wrapping_add(s[0], s[3]).rotate_left(23), s[0]);
	advance(s);
	return result;
}
#[inline]
fn next_plus(s: &mut [u64; 4]) -> u64 {
	let result = u64::wrapping_add(s[0], s[3]);
	advance(s);
	return result;
}
#[inline(always)]
fn jump(s: &mut [u64; 4]) {
	static JUMP: [u64; 4] = [0x180ec6d33cfd0aba, 0xd5a61266f0c9392c, 0xa9582618e03fc9aa, 0x39abdc4529b1661c];

	let mut s0 = 0;
	let mut s1 = 0;
	let mut s2 = 0;
	let mut s3 = 0;
	for i in 0..4 {
		for b in 0..64 {
			if (JUMP[i] & (1 << b)) != 0 {
				s0 ^= s[0];
				s1 ^= s[1];
				s2 ^= s[2];
				s3 ^= s[3];
			}
			advance(s);
		}
	}
	s[0] = s0;
	s[1] = s1;
	s[2] = s2;
	s[3] = s3;
}
