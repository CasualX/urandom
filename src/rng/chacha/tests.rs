use super::*;

// 256-bit-key vectors from draft-strombergson-chacha-test-vectors-01, TC1.
// https://datatracker.ietf.org/doc/html/draft-strombergson-chacha-test-vectors-01
#[test]
fn reduced_round_test_vectors() {
	#[track_caller]
	fn check<const N: usize>(expected: [u32; 16]) {
		let mut state = ChaChaState::<N>::new([0; 8], 0, 0);
		let mut result = Default::default();
		chacha_block(&mut state, &mut result);
		assert_eq!(expected, result[0]);
	}

	check::<8>([
		0x2fef003e, 0xd6405f89, 0xe8b85b7f, 0xa1a5091f,
		0xc30e842c, 0x3b7f9ace, 0x88e11b18, 0x1e1a71ef,
		0x72e14c98, 0x416f21b9, 0x6753449f, 0x19566d45,
		0xa3424a31, 0x01b086da, 0xb8fd7b38, 0x42fe0c0e,
	]);
	check::<12>([
		0x6a9af49b, 0x53f95507, 0x12ce1f81, 0xd583265f,
		0xbbc32904, 0x1474e049, 0xa589007e, 0x5f15ae2e,
		0x79f86405, 0xc0e37ad2, 0x3428e82c, 0x798cfaac,
		0x2c9f623a, 0x1969dea0, 0x2fe80b61, 0xbe261341,
	]);
	check::<20>([
		0xade0b876, 0x903df1a0, 0xe56a5d40, 0x28bd8653,
		0xb819d2bd, 0x1aed8da0, 0xccef36a8, 0xc70d778b,
		0x7c5941da, 0x8d485751, 0x3fe02477, 0x374ad8b8,
		0xf4b8436a, 0x1ca11815, 0x69b687c3, 0x8665eeb2,
	]);
}

#[track_caller]
fn compare_with_slp<const N: usize>(backend: fn(&mut ChaChaState<N>, &mut [[u32; 16]; CN])) {
	let states = [
		ChaChaState::new([0; 8], 0, 0),
		ChaChaState::new([u32::MAX; 8], 1, u64::MAX),
		ChaChaState::new(
			[0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c],
			0x0123456789abcdef,
			0xfedcba9876543210,
		),
		ChaChaState::new([0x55555555, 0xaaaaaaaa, 0xdeadbeef, 0x01234567, 0x89abcdef, 1, 2, 3], u64::MAX - 4, 42),
	];

	for state in states {
		let mut expected_state = state.clone();
		let mut expected = Default::default();
		slp::block(&mut expected_state, &mut expected);

		let mut actual_state = state;
		let mut actual = Default::default();
		backend(&mut actual_state, &mut actual);

		assert_eq!(actual, expected);
		assert_eq!(actual_state.get_counter(), expected_state.get_counter());
		assert_eq!(actual_state.get_stream(), expected_state.get_stream());
	}
}

#[test]
fn selected_backend_matches_slp() {
	compare_with_slp::<8>(chacha_block);
	compare_with_slp::<12>(chacha_block);
	compare_with_slp::<20>(chacha_block);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "sse2"))]
#[test]
fn sse2_matches_slp() {
	compare_with_slp::<8>(sse2::block);
	compare_with_slp::<12>(sse2::block);
	compare_with_slp::<20>(sse2::block);
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2"))]
#[test]
fn avx2_matches_slp() {
	compare_with_slp::<8>(avx2::block);
	compare_with_slp::<12>(avx2::block);
	compare_with_slp::<20>(avx2::block);
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[test]
fn wasm32_simd128_matches_slp() {
	compare_with_slp::<8>(wasm32::block);
	compare_with_slp::<12>(wasm32::block);
	compare_with_slp::<20>(wasm32::block);
}

#[test]
fn counter_wraps() {
	let mut state = ChaChaState::<20>::new([0; 8], u64::MAX - 3, 0);
	let mut result = Default::default();
	chacha_block(&mut state, &mut result);
	assert_eq!(state.get_counter(), 0);
}

#[test]
fn stream_selector_wraps() {
	let mut state = ChaChaState::<20>::new([0; 8], 0, u64::MAX);
	BlockRng::jump(&mut state);
	assert_eq!(state.get_stream(), 0);
}

#[test]
fn debug_redacts_secret_state() {
	let state = ChaChaState::<20>::new([0xdeadbeef; 8], 1, 0);
	let rng = ChaChaRng { inner: BlockRngImpl::new(state) };
	let debug = format!("{rng:?}");
	assert_eq!(debug, "ChaChaRng { .. }");
	assert!(!debug.contains("deadbeef"));
}

#[test]
fn chacha20_test_vectors() {
	#[track_caller]
	fn check(state: ChaChaState<20>, expected: [u32; 16]) {
		let mut result = Default::default();
		chacha_block(&mut state.clone(), &mut result);
		println!("state: {:?},\nresult: [\n\t{:x?},\n\t{:x?},\n\t{:x?},\n\t{:x?},\n]", state, result[0], result[1], result[2], result[3]);
		assert_eq!(expected, result[0]);
	}
	check(
		ChaChaState::new([0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c], 0x0900000000000001, 0x000000004a000000),
		[
			0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3,
			0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3,
			0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9,
			0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2,
		]
	);
	check(
		ChaChaState::new([0, 0, 0, 0, 0, 0, 0, 0], 0, 0),
		[
			0xade0b876, 0x903df1a0, 0xe56a5d40, 0x28bd8653,
			0xb819d2bd, 0x1aed8da0, 0xccef36a8, 0xc70d778b,
			0x7c5941da, 0x8d485751, 0x3fe02477, 0x374ad8b8,
			0xf4b8436a, 0x1ca11815, 0x69b687c3, 0x8665eeb2,
		]
	);
	check(
		ChaChaState::new([0, 0, 0, 0, 0, 0, 0, 0], 1, 0),
		[
			0xbee7079f, 0x7a385155, 0x7c97ba98, 0x0d082d73,
			0xa0290fcb, 0x6965e348, 0x3e53c612, 0xed7aee32,
			0x7621b729, 0x434ee69c, 0xb03371d5, 0xd539d874,
			0x281fed31, 0x45fb0a51, 0x1f0ae1ac, 0x6f4d794b,
		]
	);
	check(
		ChaChaState::new([0, 0, 0, 0, 0, 0, 0, 0x01000000], 1, 0),
		[
			0x2452eb3a, 0x9249f8ec, 0x8d829d9b, 0xddd4ceb1,
			0xe8252083, 0x60818b01, 0xf38422b8, 0x5aaa49c9,
			0xbb00ca8e, 0xda3ba7b4, 0xc4b592d1, 0xfdf2732f,
			0x4436274e, 0x2561b3c8, 0xebdd4aa6, 0xa0136c00,
		]
	);
	check(
		ChaChaState::new([0x0000ff00, 0, 0, 0, 0, 0, 0, 0], 2, 0),
		[
			0xfb4dd572, 0x4bc42ef1, 0xdf922636, 0x327f1394,
			0xa78dea8f, 0x5e269039, 0xa1bebbc1, 0xcaf09aae,
			0xa25ab213, 0x48a6b46c, 0x1b9d9bcb, 0x092c5be6,
			0x546ca624, 0x1bec45d5, 0x87f47473, 0x96f0992e,
		]
	);
	check(
		ChaChaState::new([0, 0, 0, 0, 0, 0, 0, 0], 0, 0x0200000000000000),
		[
			0x374dc6c2, 0x3736d58c, 0xb904e24a, 0xcd3f93ef,
			0x88228b1a, 0x96a4dfb3, 0x5b76ab72, 0xc727ee54,
			0x0e0e978a, 0xf3145c95, 0x1b748ea8, 0xf786c297,
			0x99c28f5f, 0x628314e8, 0x398a19fa, 0x6ded1b53,
		]
	);
}

#[test]
fn test_randomness() {
	let mut rand = ChaCha20Rng::new();
	let mut words1 = [0; 16 * CN];
	for i in 0..16 * CN {
		words1[i] = rand.next_u32();
	}
	let mut words2 = [0; 16 * CN];
	for i in 0..16 * CN {
		words2[i] = rand.next_u32();
	}
	assert_ne!(words1, words2);
}

#[test]
fn test_fill_bytes() {
	crate::rng::tests::check_fill_bytes(&mut ChaCha20Rng::new());
}

#[test]
fn fill_bytes_uses_little_endian_word_order() {
	let expected = [
		0xbee7079f, 0x7a385155, 0x7c97ba98, 0x0d082d73,
		0xa0290fcb, 0x6965e348, 0x3e53c612, 0xed7aee32,
		0x7621b729, 0x434ee69c, 0xb03371d5, 0xd539d874,
		0x281fed31, 0x45fb0a51, 0x1f0ae1ac, 0x6f4d794b,
	].map(u32::to_le_bytes);

	let mut rand = ChaCha20Rng::from_seed([0; 8]);
	let mut output = [0u8; 64];
	rand.fill_bytes(&mut output);

	assert_eq!(output, expected.as_flattened());
}

#[inline]
fn stable_digest(bytes: &[u8]) -> u64 {
	bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
		(hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
	})
}

#[test]
fn fill_block_boundary_vectors() {
	let expected = [
		(255, 0x6c41_6acb_0be4_591b, 0xc90c_bac1_9958_3d34),
		(256, 0x7fe9_0e7c_2491_dbc3, 0x3689_639d_33fe_74a6),
		(257, 0xeadf_01f2_23d7_cc9f, 0xcf36_8963_9d33_fe74),
		(512, 0x57d6_0f31_ca02_d12a, 0x3689_639d_33fe_74a6),
		(513, 0x438d_5d9a_42ca_10e4, 0xcf36_8963_9d33_fe74),
	];

	let mut master = ChaCha12Rng::from_seed_u64(42);
	let _ = master.next_u32();

	for (len, expected_digest, expected_next) in expected {
		let mut rand = master.clone();
		let mut bytes = [0u8; 513];
		rand.fill_bytes(&mut bytes[..len]);
		assert_eq!(stable_digest(&bytes[..len]), expected_digest, "length {len}");
		assert_eq!(rand.next_u64(), expected_next, "next value after length {len}");
	}
}

#[cfg(feature = "serde")]
#[test]
fn serde() {
	crate::rng::tests::check_serde_initial_state(ChaCha12Rng::new());
	crate::rng::tests::check_serde_middle_state(ChaCha12Rng::new());
}

#[cfg(feature = "serde")]
#[test]
fn serde_cache_fields_are_atomic() {
	let mut rand = ChaCha12Rng::from_seed_u64(42);
	let initial = serde_json::to_value(&rand).unwrap();
	assert!(initial.get("index").is_none());
	assert!(initial.get("random").is_none());

	let _ = rand.next_u64();
	let middle = serde_json::to_value(&rand).unwrap();
	assert!(middle.get("index").is_some());
	assert!(middle.get("random").is_some());
	assert_eq!(middle["random"][0][0], 631540493);

	rand.jump();
	let jumped = serde_json::to_value(&rand).unwrap();
	assert!(jumped.get("index").is_none());
	assert!(jumped.get("random").is_none());

	let expected = rand.clone().next_u64();
	let mut restored: crate::Random<ChaCha12Rng> = serde_json::from_value(jumped.clone()).unwrap();
	assert_eq!(restored.next_u64(), expected);
}

#[cfg(feature = "serde")]
#[test]
fn serde_rejects_partial_cache() {
	let mut rand = ChaCha12Rng::from_seed_u64(42);
	let _ = rand.next_u64();
	let saved = serde_json::to_value(rand).unwrap();

	let mut missing_random = saved.clone();
	missing_random.as_object_mut().unwrap().remove("random");
	assert!(serde_json::from_value::<ChaCha12Rng>(missing_random).is_err());

	let mut missing_index = saved;
	missing_index.as_object_mut().unwrap().remove("index");
	assert!(serde_json::from_value::<ChaCha12Rng>(missing_index).is_err());
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_v1_middle_state() {
	let saved = include_str!("chacha12-midstate-v1.json");
	let mut rand: crate::Random<ChaCha12Rng> = serde_json::from_str(saved).unwrap();
	assert_eq!(rand.next_u64(), 9316889689305211805);
}
