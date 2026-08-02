use super::*;

#[derive(Clone, Copy)]
pub struct ReproVector {
	pub u32_value: u32,
	pub u64_value: u64,
	pub f32_bits: u32,
	pub f64_bits: u64,
	pub bytes: [u8; 17],
	pub after_jump: u64,
	pub fork_left: u64,
	pub fork_right: u64,
}

impl ReproVector {
	#[track_caller]
	pub fn check<R: JumpRng>(&self, mut rand: Random<R>) {
		let expected = self;

		assert_eq!(rand.next_u32(), expected.u32_value, "next_u32");
		assert_eq!(rand.next_u64(), expected.u64_value, "next_u64");
		assert_eq!(rand.next_f32().to_bits(), expected.f32_bits, "next_f32");
		assert_eq!(rand.next_f64().to_bits(), expected.f64_bits, "next_f64");

		let mut bytes = [0u8; 17];
		rand.fill_bytes(&mut bytes);
		assert_eq!(bytes, expected.bytes, "fill_bytes");

		rand.jump();
		assert_eq!(rand.next_u64(), expected.after_jump, "jump");

		let (mut left, mut right) = rand.fork();
		assert_eq!(left.next_u64(), expected.fork_left, "fork left");
		assert_eq!(right.next_u64(), expected.fork_right, "fork right");
	}
}

#[test]
fn stable_root_constructors() {
	let _: fn(u64) -> Random<Xoshiro256Rng> = crate::seeded;

	#[cfg(feature = "getrandom")]
	{
		let _: fn() -> Random<ChaCha12Rng> = crate::new;
	}
}

#[cfg(feature = "serde")]
#[test]
fn serde_state_reproducibility_vectors() {
	const SPLITTABLE: &str = r#"{"state":11400714819323198527,"gamma":11400714819323198485}"#;
	const WYRAND: &str = r#"{"state":3257665815644502223}"#;
	const XOSHIRO: &str = r#"{"state":[14781993660996615170,15162131492471177412,4387123674275312071,9390829987253819269]}"#;
	const CHACHA: &str = r#"{"state":[42,0,42,0,42,0,42,0,1,0,0,0]}"#;

	let mut splittable = SplittableRandom::from_seed_u64(42);
	let _ = splittable.next_u32();
	assert_eq!(serde_json::to_string(&splittable).unwrap(), SPLITTABLE);
	let mut splittable: SplittableRandom = serde_json::from_str(SPLITTABLE).unwrap();
	assert_eq!(splittable.next_u64(), 0x28ef_e333_b266_f103);

	let mut wyrand = WyrandRng::from_seed_u64(42);
	let _ = wyrand.next_u32();
	assert_eq!(serde_json::to_string(&wyrand).unwrap(), WYRAND);
	let mut wyrand: WyrandRng = serde_json::from_str(WYRAND).unwrap();
	assert_eq!(wyrand.next_u64(), 0x7e5b_a615_5208_5fc6);

	let mut xoshiro = Xoshiro256Rng::from_seed_u64(42);
	let _ = xoshiro.next_u32();
	assert_eq!(serde_json::to_string(&xoshiro).unwrap(), XOSHIRO);
	let mut xoshiro: Xoshiro256Rng = serde_json::from_str(XOSHIRO).unwrap();
	assert_eq!(xoshiro.next_u64(), 0x519e_4174_576f_3791);

	assert_eq!(serde_json::to_string(&ChaCha8Rng::from_seed_u64(42)).unwrap(), CHACHA);
	assert_eq!(serde_json::to_string(&ChaCha12Rng::from_seed_u64(42)).unwrap(), CHACHA);
	assert_eq!(serde_json::to_string(&ChaCha20Rng::from_seed_u64(42)).unwrap(), CHACHA);

	let mut chacha8: ChaCha8Rng = serde_json::from_str(CHACHA).unwrap();
	let mut chacha12: ChaCha12Rng = serde_json::from_str(CHACHA).unwrap();
	let mut chacha20: ChaCha20Rng = serde_json::from_str(CHACHA).unwrap();
	assert_eq!(chacha8.next_u64(), 0x2adf_5af2_8e8c_7b1b);
	assert_eq!(chacha12.next_u64(), 0x33fe_74a6_25a4_8b0d);
	assert_eq!(chacha20.next_u64(), 0xbadf_9172_673a_7168);
}

#[cfg(feature = "getrandom")]
#[test]
fn test_trait_object() {
	// Ensure Rng is usable as a trait object
	fn test(rand: &mut Random<dyn Rng>) {
		let _: i32 = rand.random();
	}
	test(&mut crate::new());
	test(&mut crate::seeded(42));
}

#[test]
fn test_split_rng() {
	fn test<R: JumpRng + Clone>(rand: &mut Random<R>) {
		let _ = rand.split();
	}
	test(&mut crate::seeded(42));
	#[cfg(feature = "getrandom")]
	test(&mut crate::new());
	test(&mut SplittableRandom::from_seed_u64(42));
	test(&mut WyrandRng::from_seed_u64(42));

	let mut rand = crate::seeded(42);
	let mut current = rand.clone();
	let mut child = rand.split();
	assert_eq!(child.next_u64(), current.next_u64());
	assert_ne!(rand.next_u64(), child.next_u64());
}

#[test]
fn test_fork_rng() {
	#[track_caller]
	fn check<R: JumpRng>(rand: Random<R>) {
		let (mut left, mut right) = rand.fork();
		assert_ne!(left.next_u64(), right.next_u64());
	}

	check(Xoshiro256Rng::from_seed_u64(42));
	check(SplittableRandom::from_seed_u64(42));
	check(WyrandRng::from_seed_u64(42));
	check(ChaCha12Rng::from_seed([0; 8]));
}

#[test]
fn recursive_forks_are_distinct() {
	#[track_caller]
	fn check<R: JumpRng>(rand: Random<R>) {
		let (left, right) = rand.fork();
		let (mut left_left, mut left_right) = left.fork();
		let (mut right_left, mut right_right) = right.fork();
		let mut values = [
			left_left.next_u64(),
			left_right.next_u64(),
			right_left.next_u64(),
			right_right.next_u64(),
		];
		values.sort_unstable();
		assert!(values.windows(2).all(|pair| pair[0] != pair[1]));
	}

	check(Xoshiro256Rng::from_seed_u64(42));
	check(SplittableRandom::from_seed_u64(42));
	check(WyrandRng::from_seed_u64(42));
	check(ChaCha12Rng::from_seed([0; 8]));
}

#[test]
fn forks_are_reproducible() {
	#[track_caller]
	fn check<R: JumpRng + Clone>(rand: Random<R>) {
		let (mut left1, mut right1) = rand.clone().fork();
		let (mut left2, mut right2) = rand.fork();
		assert_eq!(left1.next_u64(), left2.next_u64());
		assert_eq!(right1.next_u64(), right2.next_u64());
	}

	check(Xoshiro256Rng::from_seed_u64(42));
	check(SplittableRandom::from_seed_u64(42));
	check(WyrandRng::from_seed_u64(42));
	check(ChaCha12Rng::from_seed([0; 8]));
}

#[cfg(feature = "getrandom")]
#[track_caller]
pub fn check_fill_bytes<R: Rng + Clone>(master: &mut Random<R>) {
	master.next_u64();
	master.next_u64();
	master.next_u64();
	master.next_u32();

	let zeroes = [0u8; 256];
	let mut old = [0u8; 256];
	for i in 1..256 {
		let mut rand = master.clone();
		let mut buf = [0u8; 256];
		rand.fill_bytes(&mut buf[..i]);

		// Check the buffer is correctlly filled
		let nzeroes = buf[..i].iter().filter(|&&b| b == 0).count();
		assert!(nzeroes < 8, "too many zeroes in {:?}", &buf[..i]);

		// Check OOB writes
		assert_eq!(buf[i..], zeroes[i..]);

		// Check the output of fill_bytes is consistent
		assert_eq!(buf[..i - 1], old[..i - 1]);
		old = buf;
	}
}

#[cfg(all(feature = "getrandom", feature = "serde"))]
#[track_caller]
pub fn check_serde_initial_state<R: Rng + serde::Serialize + for<'de> serde::Deserialize<'de>>(mut rand: Random<R>) {
	let saved = serde_json::to_string(&rand).unwrap();
	let v1 = rand.next_u64();
	let mut restored: R = serde_json::from_str(&saved).unwrap();
	let v2 = restored.next_u64();
	assert_eq!(v1, v2);
}

#[cfg(all(feature = "getrandom", feature = "serde"))]
#[track_caller]
pub fn check_serde_middle_state<R: Rng + serde::Serialize + for<'de> serde::Deserialize<'de>>(mut rand: Random<R>) {
	let _ = rand.next_u64();
	let saved = serde_json::to_string(&rand).unwrap();
	let v1 = rand.next_u64();
	let mut restored: R = serde_json::from_str(&saved).unwrap();
	let v2 = restored.next_u64();
	assert_eq!(v1, v2);
}
