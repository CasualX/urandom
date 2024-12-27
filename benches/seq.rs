#![feature(test)]

extern crate test;

use std::mem::size_of;
use test::Bencher;

const RAND_BENCH_N: u64 = 1000;

#[bench]
fn shuffle_100(b: &mut Bencher) {
	let mut rand = urandom::new();
	let mut x = [1usize; 100];
	b.iter(|| {
		rand.shuffle(&mut x);
		x[0]
	})
}

#[bench]
fn choose_1_of_1000(b: &mut Bencher) {
	let mut rand = urandom::new();
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
fn single_from_1000(b: &mut Bencher) {
	let mut rand = urandom::new();
	let mut x = [1usize; 1000];
	for i in 0..1000 {
		x[i] = i;
	}
	b.iter(|| {
		let mut s = 0;
		for _ in 0..RAND_BENCH_N {
			s += rand.single(&x).unwrap();
		}
		s
	});
	b.bytes = size_of::<usize>() as u64 * crate::RAND_BENCH_N;
}

macro_rules! multiple {
	($name:ident, $amount:expr, $length:expr) => {
		#[bench]
		fn $name(b: &mut Bencher) {
			let mut rand = urandom::new();
			let x = [$amount; $length];
			let mut result = [0; $amount];
			b.iter(|| {
				rand.multiple(x.iter().cloned(), &mut result);
				result[$amount - 1]
			})
		}
	};
}

multiple!(multiple_1_of_1000, 1, 1000);
multiple!(multiple_950_of_1000, 950, 1000);
multiple!(multiple_10_of_100, 10, 100);
multiple!(multiple_90_of_100, 90, 100);
