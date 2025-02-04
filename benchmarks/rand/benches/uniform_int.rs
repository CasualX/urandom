#![feature(test)]

extern crate test;

use rand::prelude::*;
use test::{black_box, Bencher};

const RAND_BENCH_N: u64 = 1000;

#[bench]
fn uniform_sample_rand(b: &mut Bencher) {
	let mut rng = SmallRng::from_os_rng();
	b.iter(|| {
		let range = black_box(rand::distr::Uniform::new(500, 20000).unwrap());
		let mut accum = 0u32;
		for _ in 0..RAND_BENCH_N {
			accum = accum.wrapping_add(rng.sample(range));
		}
		accum
	});
}

#[bench]
fn uniform_sample_urandom(b: &mut Bencher) {
	let mut rand = urandom::new();
	b.iter(|| {
		let range = black_box(urandom::distr::Uniform::new(500, 20000));
		let mut accum = 0u32;
		for _ in 0..RAND_BENCH_N {
			accum = accum.wrapping_add(rand.sample(&range));
		}
		accum
	});
}

#[bench]
fn uniform_range_rand(b: &mut Bencher) {
	let mut rng = SmallRng::from_os_rng();
	b.iter(|| {
		let mut accum = 0u32;
		for _ in 0..RAND_BENCH_N {
			accum = accum.wrapping_add(rng.random_range(500..20000));
		}
		accum
	});
}

#[bench]
fn uniform_range_urandom(b: &mut Bencher) {
	let mut rand = urandom::new();
	b.iter(|| {
		let mut accum = 0u32;
		for _ in 0..RAND_BENCH_N {
			accum = accum.wrapping_add(rand.range(500..20000));
		}
		accum
	});
}
