#![feature(test)]

extern crate test;

use std::mem::size_of;
use test::{black_box, Bencher};
use urandom::rng::{SplitMix64, Xoshiro256, ChaCha20};

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;

macro_rules! fill_bytes {
	($fnn:ident, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rng = $gen;
			let mut buf = [0u8; BYTES_LEN];
			b.iter(|| {
				for _ in 0..RAND_BENCH_N {
					rng.fill_bytes(&mut buf);
					black_box(buf);
				}
			});
			b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
		}
	};
}

fill_bytes!(fill_bytes_splitmix64, SplitMix64::new());
fill_bytes!(fill_bytes_xoshiro256, Xoshiro256::new());
fill_bytes!(fill_bytes_chacha20, ChaCha20::new());

macro_rules! fill_u32 {
	($fnn:ident, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rng = $gen;
			let mut buf = [0u32; BYTES_LEN / 4];
			b.iter(|| {
				for _ in 0..RAND_BENCH_N {
					rng.fill_u32(&mut buf);
					black_box(buf);
				}
			});
			b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
		}
	};
}

fill_u32!(fill_u32_splitmix64, SplitMix64::new());
fill_u32!(fill_u32_xoshiro256, Xoshiro256::new());
fill_u32!(fill_u32_chacha20, ChaCha20::new());

macro_rules! fill_u64 {
	($fnn:ident, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rng = $gen;
			let mut buf = [0u64; BYTES_LEN / 8];
			b.iter(|| {
				for _ in 0..RAND_BENCH_N {
					rng.fill_u64(&mut buf);
					black_box(buf);
				}
			});
			b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
		}
	};
}

fill_u64!(fill_u64_splitmix64, SplitMix64::new());
fill_u64!(fill_u64_xoshiro256, Xoshiro256::new());
fill_u64!(fill_u64_chacha20, ChaCha20::new());

macro_rules! next_uint {
	($fnn:ident, $ty:ty, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rng = $gen;
			b.iter(|| {
				let mut accum: $ty = 0;
				for _ in 0..RAND_BENCH_N {
					accum = accum.wrapping_add(rng.next::<$ty>());
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

next_uint!(next_u32_splitmix64, u32, SplitMix64::new());
next_uint!(next_u32_xoshiro256, u32, Xoshiro256::new());
next_uint!(next_u32_chacha20, u32, ChaCha20::new());

next_uint!(next_u64_splitmix64, u64, SplitMix64::new());
next_uint!(next_u64_xoshiro256, u64, Xoshiro256::new());
next_uint!(next_u64_chacha20, u64, ChaCha20::new());

macro_rules! next_float {
	($fnn:ident, $ty:ty, $gen:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rng = $gen;
			b.iter(|| {
				let mut accum: $ty = 0.0;
				for _ in 0..RAND_BENCH_N {
					accum += rng.next::<$ty>();
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

next_float!(next_f32_splitmix64, f32, SplitMix64::new());
next_float!(next_f32_xoshiro256, f32, Xoshiro256::new());
next_float!(next_f32_chacha20, f32, ChaCha20::new());

next_float!(next_f64_splitmix64, f64, SplitMix64::new());
next_float!(next_f64_xoshiro256, f64, Xoshiro256::new());
next_float!(next_f64_chacha20, f64, ChaCha20::new());

macro_rules! init {
	($fnn:ident, $gen:path) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			b.iter(|| {
				<$gen>::new()
			});
		}
	};
}

init!(init_splitmix64, SplitMix64);
init!(init_xoshiro256, Xoshiro256);
init!(init_chacha20, ChaCha20);
