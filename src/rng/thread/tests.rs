use super::*;

#[test]
fn scalar_output_reseeds_at_threshold() {
	let mut state = ThreadRngState::new();
	state.bytes_until_reseed = 8;

	let _ = state.generate(8, Rng::next_u64);
	assert_eq!(state.bytes_until_reseed, 0);

	let _ = state.generate(4, Rng::next_u32);
	assert_eq!(state.bytes_until_reseed, RESEED_THRESHOLD - 4);
}

#[test]
fn bulk_output_reseeds_inside_fill() {
	let mut state = ThreadRngState::new();
	state.bytes_until_reseed = 3;
	let mut bytes = [MaybeUninit::uninit(); 5];

	state.fill_bytes(&mut bytes);

	assert_eq!(state.bytes_until_reseed, RESEED_THRESHOLD - 2);
}

#[test]
fn manual_reseed_resets_threshold() {
	let mut state = ThreadRngState::new();
	state.bytes_until_reseed = 1;

	state.reseed();

	assert_eq!(state.bytes_until_reseed, RESEED_THRESHOLD);
}

#[test]
fn empty_fill_does_not_reseed() {
	let mut state = ThreadRngState::new();
	state.bytes_until_reseed = 0;

	state.fill_bytes(&mut []);

	assert_eq!(state.bytes_until_reseed, 0);
}
