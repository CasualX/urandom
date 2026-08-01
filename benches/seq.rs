#![feature(test)]

extern crate test;

use std::mem::size_of;
use test::Bencher;

const RAND_BENCH_N: u64 = 1000;

#[bench]
fn shuffle_100(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();
	let mut x = [1usize; 100];
	b.iter(|| {
		rand.shuffle(&mut x);
		x[0]
	})
}

#[bench]
fn choose_1_of_1000(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();
	let mut x = [1usize; 1000];
	for i in 0..1000 {
		x[i] = i;
	}
	b.iter(|| {
		let mut s = 0;
		for _ in 0..RAND_BENCH_N {
			s += rand.choose(&x).unwrap();
		}
		s
	});
	b.bytes = size_of::<usize>() as u64 * crate::RAND_BENCH_N;
}

#[bench]
fn choose_iter_from_1000(b: &mut Bencher) {
	let mut rand = urandom::rng::Xoshiro256Rng::new();
	let mut x = [1usize; 1000];
	for i in 0..1000 {
		x[i] = i;
	}
	b.iter(|| {
		let mut s = 0;
		for _ in 0..RAND_BENCH_N {
			s += rand.choose_iter(&x).unwrap();
		}
		s
	});
	b.bytes = size_of::<usize>() as u64 * crate::RAND_BENCH_N;
}

macro_rules! choose_multiple {
	($name:ident, $amount:expr, $length:expr) => {
		#[bench]
		fn $name(b: &mut Bencher) {
			let mut rand = urandom::rng::Xoshiro256Rng::new();
			let x = [$amount; $length];
			let mut result = [0; $amount];
			b.iter(|| {
				rand.choose_multiple(x.iter().cloned(), &mut result);
				result[$amount - 1]
			})
		}
	};
}

choose_multiple!(choose_multiple_1_of_1000, 1, 1000);
choose_multiple!(choose_multiple_950_of_1000, 950, 1000);
choose_multiple!(choose_multiple_10_of_100, 10, 100);
choose_multiple!(choose_multiple_90_of_100, 90, 100);
