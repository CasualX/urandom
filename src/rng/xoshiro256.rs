use super::*;

/// The xoshiro256 random number generator[^1].
///
/// It has excellent (sub-ns) speed, a state (256 bits) that is large enough for any parallel application, and it passes all tests we are aware of.
///
/// [^1]: David Blackman and Sebastiano Vigna, 2021. [*Scrambled Linear Pseudorandom Number Generators*](https://vigna.di.unimi.it/ftp/papers/ScrambledLinear.pdf).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Xoshiro256Rng {
	state: [u64; 4],
}

impl Xoshiro256Rng {
	/// Creates a new instance seeded from system entropy.
	///
	/// This method is the recommended way to construct PRNGs since it is convenient and securely seeded.
	///
	/// # Panics
	///
	/// If [`getentropy`] is unable to provide secure entropy this method will panic.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::Xoshiro256Rng::new();
	/// let value: i32 = rand.random();
	/// ```
	#[inline]
	#[cfg(feature = "getrandom")]
	pub fn new() -> Random<Xoshiro256Rng> {
		Self::from_seed(util::getrandom())
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
	#[cfg_attr(feature = "getrandom", doc = "let mut master = urandom::rng::SplitMix64Rng::new();")]
	#[cfg_attr(not(feature = "getrandom"), doc = "let mut master = urandom::rng::SplitMix64Rng::from_seed_u64(42);")]
	/// let mut rand = urandom::rng::Xoshiro256Rng::from_rng(&mut master);
	/// let value: i32 = rand.random();
	/// ```
	#[inline]
	pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<Xoshiro256Rng> {
		let seed = core::array::from_fn(|_| rand.next_u64());
		Self::from_seed(seed)
	}

	/// Creates a new instance directly from its native 256-bit seed.
	///
	/// Callers are responsible for supplying a suitable seed; in particular, the all-zero seed causes the generator to produce only zeros.
	///
	/// # Examples
	///
	/// ```
	/// let seed = [1, 2, 3, 4];
	/// let mut rand = urandom::rng::Xoshiro256Rng::from_seed(seed);
	/// let value: u64 = rand.random();
	/// ```
	#[inline]
	pub fn from_seed(seed: [u64; 4]) -> Random<Xoshiro256Rng> {
		Random::from(Xoshiro256Rng { state: seed })
	}

	/// Creates a reproducible instance by expanding a 64-bit seed into the native 256-bit seed.
	///
	/// The seed expansion and resulting stream are covered by the [reproducibility guarantee](crate::rng#reproducibility-guarantee).
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::Xoshiro256Rng::from_seed_u64(42);
	/// let value: u32 = rand.random();
	/// assert_eq!(value, 368317477);
	/// ```
	pub fn from_seed_u64(seed: u64) -> Random<Xoshiro256Rng> {
		let mut master = SplitMix64Rng::from_seed_u64(seed);
		let state = [master.next_u64(), master.next_u64(), master.next_u64(), master.next_u64()];
		Random::from(Xoshiro256Rng { state })
	}
}

impl Sealed for Xoshiro256Rng {}

impl Rng for Xoshiro256Rng {
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
}

impl JumpRng for Xoshiro256Rng {
	/// Advances the generator by 2<sup>128</sup> state transitions.
	#[inline(never)]
	fn jump(&mut self) {
		jump(&mut self.state)
	}

	/// Derives two new states using separately mixed output material.
	#[inline(never)]
	fn fork(mut self) -> (Self, Self) {
		const WORD_DOMAIN: u64 = super::splitmix64::GOLDEN_GAMMA;
		const LEFT_DOMAIN: u64 = WORD_DOMAIN;
		const RIGHT_DOMAIN: u64 = WORD_DOMAIN.wrapping_mul(5);

		let mut derive = |domain: u64| {
			let state = core::array::from_fn(|index| {
				let domain = domain.wrapping_add((index as u64).wrapping_mul(WORD_DOMAIN));
				splitmix64::mix64(self.next_u64().wrapping_add(domain))
			});
			Xoshiro256Rng { state }
		};

		(derive(LEFT_DOMAIN), derive(RIGHT_DOMAIN))
	}
}

#[test]
fn fork_reseeds_degenerate_zero_state() {
	let rand = Xoshiro256Rng { state: [0; 4] };
	let (left, right) = rand.fork();
	assert_ne!(left.state, [0; 4]);
	assert_ne!(right.state, [0; 4]);
	assert_ne!(left.state, right.state);
}

#[cfg(feature = "getrandom")]
#[test]
fn fill_bytes() {
	crate::rng::tests::check_fill_bytes(&mut Xoshiro256Rng::new());
}

#[cfg(feature = "getrandom")]
#[cfg(feature = "serde")]
#[test]
fn serde() {
	tests::check_serde_initial_state(Xoshiro256Rng::new());
	tests::check_serde_middle_state(Xoshiro256Rng::new());
}

#[test]
fn test_from_seed_u64() {
	tests::ReproVector {
		u32_value: 0x15f4_1425,
		u64_value: 0x519e_4174_576f_3791,
		f32_bits: 0x3f88_0249,
		f64_bits: 0x3ff4_e743_fce4_9537,
		bytes: [0x73, 0x6a, 0x84, 0x74, 0x38, 0x1c, 0x23, 0xcb, 0x7d, 0xde, 0x50, 0x4e, 0x00, 0x9f, 0x8d, 0x96, 0x56],
		after_jump: 0x6bab_2441_bc00_1e39,
		fork_left: 0xef37_c207_4f11_4358,
		fork_right: 0xa79c_eb61_e325_5531,
	}.check(Xoshiro256Rng::from_seed_u64(42));
}

#[test]
fn test_from_seed() {
	tests::ReproVector {
		u32_value: 0x0000_0000,
		u64_value: 0x0000_0000_0380_0067,
		f32_bits: 0x3f80_0060,
		f64_bits: 0x3ff8_0016_0001_8040,
		bytes: [0xcd, 0x33, 0xc4, 0x9a, 0x01, 0xa2, 0x12, 0x80, 0xba, 0x33, 0xee, 0xcd, 0x8a, 0x97, 0x69, 0x8a, 0xbd],
		after_jump: 0x6240_8345_715a_13e6,
		fork_left: 0x0351_a198_3194_e5d1,
		fork_right: 0x114a_cc21_ed10_b213,
	}.check(Xoshiro256Rng::from_seed([1, 2, 3, 4]));
}

//----------------------------------------------------------------
// Xoshiro256Rng implementation details

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
