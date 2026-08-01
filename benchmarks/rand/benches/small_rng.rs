#![feature(test)]

extern crate test;

use core::mem::size_of;
use rand::prelude::*;
use test::{black_box, Bencher};

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;

#[bench]
fn fill_bytes_rand(b: &mut Bencher) {
	let mut rng: SmallRng = rand::make_rng();
	let mut buf = [0u8; BYTES_LEN];

	b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			rng.fill_bytes(&mut buf);
			black_box(&buf);
		}
	});
}

#[bench]
fn fill_bytes_urandom(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();
	let mut buf = [0u8; BYTES_LEN];

	b.bytes = BYTES_LEN as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			rand.fill_bytes(&mut buf);
			black_box(&buf);
		}
	});
}

#[bench]
fn u32_rand(b: &mut Bencher) {
	let mut rng: SmallRng = rand::make_rng();

	b.bytes = size_of::<u32>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value = rng.next_u32();
			black_box(value);
		}
	});
}

#[bench]
fn u32_urandom(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();

	b.bytes = size_of::<u32>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: u32 = rand.random();
			black_box(value);
		}
	});
}

#[bench]
fn u64_rand(b: &mut Bencher) {
	let mut rng: SmallRng = rand::make_rng();

	b.bytes = size_of::<u64>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value = rng.next_u64();
			black_box(value);
		}
	});
}

#[bench]
fn u64_urandom(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();

	b.bytes = size_of::<u64>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: u64 = rand.random();
			black_box(value);
		}
	});
}

#[bench]
fn f64_rand(b: &mut Bencher) {
	let mut rng: SmallRng = rand::make_rng();

	b.bytes = size_of::<f64>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: f64 = rng.random();
			black_box(value);
		}
	});
}

#[bench]
fn f64_urandom(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();

	b.bytes = size_of::<f64>() as u64 * RAND_BENCH_N;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: f64 = rand.random();
			black_box(value);
		}
	});
}
