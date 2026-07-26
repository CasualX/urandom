#![feature(test)]

extern crate test;

use std::mem::size_of;
use test::{black_box, Bencher};
use urandom::rng::*;

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;

macro_rules! fill_bytes {
	($fnn:ident, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = $gen;
			let mut buf = [0u8; BYTES_LEN];
			b.iter(|| {
				for _ in 0..RAND_BENCH_N {
					rand.fill_bytes(&mut buf);
					black_box(buf);
				}
			});
			b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
		}
	};
}

fill_bytes!(fill_bytes_xoshiro256, Xoshiro256Rng::new());
fill_bytes!(fill_bytes_splitmix64, SplitMix64Rng::new());
// fill_bytes!(fill_bytes_wyrand, WyrandRng::new());
fill_bytes!(fill_bytes_chacha8, ChaCha8Rng::new());
fill_bytes!(fill_bytes_chacha12, ChaCha12Rng::new());
fill_bytes!(fill_bytes_chacha20, ChaCha20Rng::new());
fill_bytes!(fill_bytes_system, SystemRng::<31>::new());

macro_rules! next_uint {
	($fnn:ident, $ty:ty, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = $gen;
			b.iter(|| {
				let mut accum: $ty = 0;
				for _ in 0..RAND_BENCH_N {
					accum = accum.wrapping_add(rand.random::<$ty>());
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

next_uint!(next_u32_xoshiro256, u32, Xoshiro256Rng::new());
next_uint!(next_u32_splitmix64, u32, SplitMix64Rng::new());
// next_uint!(next_u32_wyrand, u32, WyrandRng::new());
next_uint!(next_u32_chacha8, u32, ChaCha8Rng::new());
next_uint!(next_u32_chacha12, u32, ChaCha12Rng::new());
next_uint!(next_u32_chacha20, u32, ChaCha20Rng::new());
next_uint!(next_u32_system, u32, SystemRng::<31>::new());

next_uint!(next_u64_xoshiro256, u64, Xoshiro256Rng::new());
next_uint!(next_u64_splitmix64, u64, SplitMix64Rng::new());
// next_uint!(next_u64_wyrand, u64, WyrandRng::new());
next_uint!(next_u64_chacha8, u64, ChaCha8Rng::new());
next_uint!(next_u64_chacha12, u64, ChaCha12Rng::new());
next_uint!(next_u64_chacha20, u64, ChaCha20Rng::new());
next_uint!(next_u64_system, u64, SystemRng::<31>::new());

macro_rules! next_float {
	($fnn:ident, $ty:ty, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = $gen;
			b.iter(|| {
				let mut accum: $ty = 0.0;
				for _ in 0..RAND_BENCH_N {
					accum += rand.random::<$ty>();
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

next_float!(next_f32_xoshiro256, f32, Xoshiro256Rng::new());
next_float!(next_f32_splitmix64, f32, SplitMix64Rng::new());
// next_float!(next_f32_wyrand, f32, WyrandRng::new());
next_float!(next_f32_chacha8, f32, ChaCha8Rng::new());
next_float!(next_f32_chacha12, f32, ChaCha12Rng::new());
next_float!(next_f32_chacha20, f32, ChaCha20Rng::new());
next_float!(next_f32_system, f32, SystemRng::<31>::new());

next_float!(next_f64_xoshiro256, f64, Xoshiro256Rng::new());
next_float!(next_f64_splitmix64, f64, SplitMix64Rng::new());
// next_float!(next_f64_wyrand, f64, WyrandRng::new());
next_float!(next_f64_chacha8, f64, ChaCha8Rng::new());
next_float!(next_f64_chacha12, f64, ChaCha12Rng::new());
next_float!(next_f64_chacha20, f64, ChaCha20Rng::new());
next_float!(next_f64_system, f64, SystemRng::<31>::new());
