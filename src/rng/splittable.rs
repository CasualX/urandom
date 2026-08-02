use super::*;

/// Fast pseudorandom number generator designed for recursive fork-join computations.
///
/// Each instance has a 64-bit seed and an independently derived odd increment. Forking advances
/// the current stream and creates a descendant with a newly mixed seed and increment, making it
/// inexpensive to build large deterministic trees of pseudorandom streams.
///
/// This is the `SplittableRandom` algorithm described by Steele, Lea, and Flood in
/// [*Fast Splittable Pseudorandom Number Generators*](https://gee.cs.oswego.edu/dl/papers/oopsla14.pdf).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SplittableRandom {
	state: u64,
	gamma: u64,
}

impl SplittableRandom {
	/// Creates a new instance seeded from system entropy.
	///
	/// This method initializes both the starting point and stream increment from entropy.
	///
	/// # Panics
	///
	/// If [`getentropy`] is unable to provide secure entropy this method will panic.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::SplittableRandom::new();
	/// let value: i32 = rand.random();
	/// ```
	#[inline]
	#[cfg(feature = "getrandom")]
	pub fn new() -> Random<SplittableRandom> {
		Self::from_seed(pod::getrandom())
	}

	/// Creates a new instance seeded from another generator.
	///
	/// Two 64-bit values initialize the starting point and stream increment.
	#[inline]
	pub fn from_rng<R: Rng + ?Sized>(rand: &mut Random<R>) -> Random<SplittableRandom> {
		let state = rand.next_u64();
		let gamma = mix_gamma(rand.next_u64());
		Random::from(SplittableRandom { state, gamma })
	}

	/// Creates an instance from 128 bits of seed material.
	///
	/// The first word is used as the starting point. The second is mixed into the stream increment,
	/// making it odd and ensuring that it has at least 24 bit transitions.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::SplittableRandom::from_seed([1, 2]);
	/// let value: u64 = rand.random();
	/// ```
	#[inline]
	pub fn from_seed(seed: [u64; 2]) -> Random<SplittableRandom> {
		let [state, gamma] = seed;
		let gamma = mix_gamma(gamma);
		Random::from(SplittableRandom { state, gamma })
	}

	/// Creates the reproducible single-seed construction described in the paper.
	///
	/// The seed is used directly as the starting point and the golden-ratio increment selects the
	/// initial stream. Descendants created by [`Random::fork`] derive their own increments.
	///
	/// The resulting stream is covered by the [reproducibility guarantee](crate::rng#reproducibility-guarantee).
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::rng::SplittableRandom::from_seed_u64(42);
	/// let value: u32 = rand.random();
	/// assert_eq!(value, 3803690062);
	/// ```
	#[inline]
	pub fn from_seed_u64(seed: u64) -> Random<SplittableRandom> {
		Random::from(SplittableRandom { state: seed, gamma: GOLDEN_GAMMA })
	}
}

impl Sealed for SplittableRandom {}

impl Rng for SplittableRandom {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		mix32(next_seed(&mut self.state, self.gamma))
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		mix64(next_seed(&mut self.state, self.gamma))
	}
	#[inline(never)]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		let mut rng = self.clone();
		util::rng_fill_bytes(&mut rng, buf);
		*self = rng;
	}
}

impl JumpRng for SplittableRandom {
	/// Advances the generator by 2<sup>40</sup> state transitions.
	#[inline]
	fn jump(&mut self) {
		self.state = self.state.wrapping_add(self.gamma << 40);
	}

	/// Continues this stream on the left and splits off a newly parameterized stream on the right.
	#[inline]
	fn fork(mut self) -> (Self, Self) {
		let state = mix64(next_seed(&mut self.state, self.gamma));
		let gamma = mix_gamma(next_seed(&mut self.state, self.gamma));
		let right = SplittableRandom { state, gamma };
		(self, right)
	}
}

#[cfg(feature = "getrandom")]
#[cfg(feature = "serde")]
#[test]
fn serde() {
	tests::check_serde_initial_state(SplittableRandom::new());
	tests::check_serde_middle_state(SplittableRandom::new());
}

#[test]
fn generated_gammas_are_odd_and_well_mixed() {
	for seed in [0, 1, 2, 3, u64::MAX, GOLDEN_GAMMA] {
		let gamma = mix_gamma(seed);
		assert_eq!(gamma & 1, 1);
		assert!((gamma ^ (gamma >> 1)).count_ones() >= 24);
	}
}

#[test]
fn from_seed_mixes_gamma() {
	for [state, gamma_seed] in [[1, 2], [4, 5], [0, 0], [u64::MAX, u64::MAX]] {
		let rand = SplittableRandom::from_seed([state, gamma_seed]);
		assert_eq!(rand.state, state);
		assert_eq!(rand.gamma, mix_gamma(gamma_seed));
		assert_eq!(rand.gamma & 1, 1);
		assert!((rand.gamma ^ (rand.gamma >> 1)).count_ones() >= 24);
	}
}

#[test]
fn test_from_seed() {
	tests::ReproVector {
		u32_value: 0xe4c6_2eb5,
		u64_value: 0x69af_57bc_0f3a_b695,
		f32_bits: 0x3faf_b365,
		f64_bits: 0x3ffb_59f4_db30_5829,
		bytes: [0xb2, 0x92, 0xb7, 0x24, 0xcd, 0x3c, 0xed, 0xc7, 0x71, 0x13, 0x1f, 0x6a, 0xf1, 0x6c, 0xaf, 0x4e, 0x4e],
		after_jump: 0xcf49_9e1c_6b09_c30e,
		fork_left: 0x0cb7_7244_bd3a_9a70,
		fork_right: 0x873a_aa0e_faa8_4051,
	}.check(SplittableRandom::from_seed([1, 2]));
}

#[test]
fn test_from_seed_u64() {
	tests::ReproVector {
		u32_value: 0xe2b7_b44e,
		u64_value: 0x28ef_e333_b266_f103,
		f32_bits: 0x3fc8_0d2a,
		f64_bits: 0x3ff5_81ce_1ff0_e4ae,
		bytes: [0xf2, 0x23, 0x48, 0x24, 0x5a, 0x58, 0xbc, 0x09, 0x06, 0xdb, 0x80, 0x3c, 0xfa, 0x31, 0x44, 0xde, 0x5d],
		after_jump: 0x315a_be1a_f4f6_78c8,
		fork_left: 0xac82_cf3b_134f_bd02,
		fork_right: 0x28f6_43a1_c345_54ee,
	}.check(SplittableRandom::from_seed_u64(42));
}

//----------------------------------------------------------------
// SplittableRandom implementation details

pub(super) const GOLDEN_GAMMA: u64 = 0x9e3779b97f4a7c15;

#[inline]
fn next_seed(state: &mut u64, gamma: u64) -> u64 {
	*state = state.wrapping_add(gamma);
	*state
}

// David Stafford's Mix13 variant, used for SplittableRandom output and SplitMix64 seed expansion.
#[inline]
pub(super) fn mix64(mut z: u64) -> u64 {
	z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
	z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
	z ^ (z >> 31)
}

// The high half of David Stafford's Mix4 variant.
#[inline]
fn mix32(mut z: u64) -> u32 {
	z = (z ^ (z >> 33)).wrapping_mul(0x62a9d9ed799705f5);
	((z ^ (z >> 28)).wrapping_mul(0xcb24d0a5c88c35b3) >> 32) as u32
}

#[inline]
fn mix_gamma(mut z: u64) -> u64 {
	// MurmurHash3's 64-bit finalizer is deliberately distinct from the output mixer.
	z = (z ^ (z >> 33)).wrapping_mul(0xff51afd7ed558ccd);
	z = (z ^ (z >> 33)).wrapping_mul(0xc4ceb9fe1a85ec53);
	z = (z ^ (z >> 33)) | 1;
	if (z ^ (z >> 1)).count_ones() < 24 {
		z ^= 0xaaaa_aaaa_aaaa_aaaa;
	}
	z
}

#[inline]
pub(super) fn splitmix64(state: &mut u64) -> u64 {
	*state = state.wrapping_add(GOLDEN_GAMMA);
	mix64(*state)
}
