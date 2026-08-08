#![no_std]

use urandom::rng::{ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};
use urandom::Rng;

const EXPECTED_FINGERPRINT: u32 = 0x07f6_7cac;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	core::arch::wasm32::unreachable()
}

fn hash_u32(mut hash: u32, value: u32) -> u32 {
	for byte in value.to_le_bytes() {
		hash ^= byte as u32;
		hash = hash.wrapping_mul(0x0100_0193);
	}
	hash
}

fn hash_bytes(mut hash: u32, bytes: &[u8]) -> u32 {
	for &byte in bytes {
		hash ^= byte as u32;
		hash = hash.wrapping_mul(0x0100_0193);
	}
	hash
}

fn hash_stream<R: Rng>(mut hash: u32, rng: &mut R) -> u32 {
	let mut index = 0;
	while index < 320 {
		hash = hash_u32(hash, rng.next_u32());
		index += 1;
	}
	hash
}

#[unsafe(no_mangle)]
pub extern "C" fn fingerprint() -> u32 {
	let seed = [
		0x0302_0100, 0x0706_0504, 0x0b0a_0908, 0x0f0e_0d0c,
		0x1312_1110, 0x1716_1514, 0x1b1a_1918, 0x1f1e_1d1c,
	];
	let mut hash = 0x811c_9dc5;
	hash = hash_stream(hash, &mut *ChaCha8Rng::from_seed(seed));
	hash = hash_stream(hash, &mut *ChaCha12Rng::from_seed(seed));
	hash = hash_stream(hash, &mut *ChaCha20Rng::from_seed(seed));

	let mut rng = ChaCha12Rng::from_seed_u64(0x0123_4567_89ab_cdef);
	let mut bytes = [0u8; 1025];
	for len in [1, 63, 64, 255, 256, 257, 1024, 1025] {
		rng.fill_bytes(&mut bytes[..len]);
		hash = hash_bytes(hash, &bytes[..len]);
	}
	hash
}

#[unsafe(no_mangle)]
pub extern "C" fn verify() -> u32 {
	u32::from(fingerprint() == EXPECTED_FINGERPRINT)
}
