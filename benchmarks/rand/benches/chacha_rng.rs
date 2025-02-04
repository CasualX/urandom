#![feature(test)]

extern crate test;

use rand::prelude::*;
use test::{black_box, Bencher};

const RAND_BENCH_N: u64 = 1000;
const BYTES_LEN: usize = 1024;

#[bench]
fn fill_bytes_rand(b: &mut Bencher) {
	let mut rng = StdRng::from_os_rng();
	let mut buf = [0u8; BYTES_LEN];

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			rng.fill_bytes(&mut buf);
			black_box(&buf);
		}
	});
}

#[bench]
fn fill_bytes_urandom(b: &mut Bencher) {
	let mut rand = urandom::csprng();
	let mut buf = [0u8; BYTES_LEN];

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			rand.fill_bytes(&mut buf);
			black_box(&buf);
		}
	});
}

#[bench]
fn u64_rand(b: &mut Bencher) {
	let mut rng = StdRng::from_os_rng();

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value = rng.next_u32();
			black_box(&value);
		}
	});
}

#[bench]
fn u64_urandom(b: &mut Bencher) {
	let mut rand = urandom::csprng();

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value = rand.next_u32();
			black_box(&value);
		}
	});
}

#[bench]
fn f64_rand(b: &mut Bencher) {
	let mut rng = StdRng::from_os_rng();

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: f64 = rng.random();
			black_box(&value);
		}
	});
}

#[bench]
fn f64_urandom(b: &mut Bencher) {
	let mut rand = urandom::csprng();

	b.bytes = BYTES_LEN as u64;
	b.iter(|| {
		for _ in 0..RAND_BENCH_N {
			let value: f64 = rand.next();
			black_box(&value);
		}
	});
}
