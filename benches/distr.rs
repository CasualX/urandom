#![feature(test)]

extern crate test;

use std::mem::size_of;
use test::{Bencher, black_box};
use urandom::distr;

const RAND_BENCH_N: u64 = 1000;

macro_rules! distr_int {
	($fnn:ident, $ty:ty, $distr:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = urandom::new();
			let distr = black_box($distr);

			b.iter(|| {
				let mut accum = 0 as $ty;
				for _ in 0..RAND_BENCH_N {
					let x: $ty = rand.sample(&distr);
					accum = accum.wrapping_add(x);
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

macro_rules! distr_float {
	($fnn:ident, $ty:ty, $distr:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = urandom::new();
			let distr = black_box($distr);

			b.iter(|| {
				let mut accum = 0.0;
				for _ in 0..RAND_BENCH_N {
					let x: $ty = rand.sample(&distr);
					accum += x;
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

macro_rules! distr_as_u32 {
	($fnn:ident, $ty:ty, $distr:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = urandom::new();
			let distr = black_box($distr);

			b.iter(|| {
				let mut accum = 0u32;
				for _ in 0..RAND_BENCH_N {
					let x: $ty = rand.sample(&distr);
					accum = accum.wrapping_add(x as u32);
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

// construct and sample from a range
macro_rules! range_int {
	($fnn:ident, $ty:ident, $low:expr, $high:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = urandom::new();

			b.iter(|| {
				let mut high = $high;
				let mut accum: $ty = 0;
				for _ in 0..RAND_BENCH_N {
					accum = accum.wrapping_add(rand.uniform($low..high));
					// force recalculation of range each time
					high = high.wrapping_add(1) & $ty::MAX;
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

// construct and sample from a floating-point range
macro_rules! range_float {
	($fnn:ident, $ty:ident, $low:expr, $high:expr) => {
		#[bench]
		fn $fnn(b: &mut Bencher) {
			let mut rand = urandom::new();

			b.iter(|| {
				let mut high = $high;
				let mut low = $low;
				let mut accum: $ty = 0.0;
				for _ in 0..RAND_BENCH_N {
					accum += rand.uniform(low..high);
					// force recalculation of range each time
					low += 0.9;
					high += 1.1;
				}
				accum
			});
			b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
		}
	};
}

distr_int!(uniform_i8, i8, distr::Uniform::from(20i8..100));
distr_int!(uniform_i16, i16, distr::Uniform::from(-500i16..2000));
distr_int!(uniform_i32, i32, distr::Uniform::from(-200_000_000i32..800_000_000));
distr_int!(uniform_i64, i64, distr::Uniform::from(3i64..123_456_789_123));
distr_int!(uniform_usize16, usize, distr::Uniform::from(0usize..0xb9d7));
distr_int!(uniform_usize32, usize, distr::Uniform::from(0usize..0x548c0f43));
#[cfg(target_pointer_width = "64")]
distr_int!(uniform_usize64, usize, distr::Uniform::from(0usize..0x3a42714f2bf927a8));
distr_int!(uniform_isize, isize, distr::Uniform::from(-1060478432isize..1858574057));

distr_float!(uniform_f32, f32, distr::Uniform::from(2.26f32..2.319));
distr_float!(uniform_f64, f64, distr::Uniform::from(2.26f64..2.319));

// standard
distr_int!(standard_i8, i8, distr::StandardUniform);
distr_int!(standard_i16, i16, distr::StandardUniform);
distr_int!(standard_i32, i32, distr::StandardUniform);
distr_int!(standard_i64, i64, distr::StandardUniform);

distr_as_u32!(standard_bool, bool, distr::StandardUniform);
distr_as_u32!(standard_alnum, char, distr::Alnum);
distr_as_u32!(standard_char, char, distr::StandardUniform);

distr_float!(standard_f32, f32, distr::StandardUniform);
distr_float!(standard_f64, f64, distr::StandardUniform);
distr_float!(float01_f32, f32, distr::Float01);
distr_float!(float01_f64, f64, distr::Float01);

// Algorithms such as Fisher–Yates shuffle often require uniform values from an
// incrementing range 0..n. We use -1..n here to prevent wrapping in the test
// from generating a 0-sized range.
range_int!(range_i8_low, i8, -1i8, 0);
range_int!(range_i16_low, i16, -1i16, 0);
range_int!(range_i32_low, i32, -1i32, 0);
range_int!(range_i64_low, i64, -1i64, 0);

// These were the initially tested ranges. They are likely to see fewer rejections than the low tests.
range_int!(range_i8_high, i8, -20i8, 100);
range_int!(range_i16_high, i16, -500i16, 2000);
range_int!(range_i32_high, i32, -200_000_000i32, 800_000_000);
range_int!(range_i64_high, i64, 3i64, 123_456_789_123);

range_float!(range_f32, f32, -20000.0f32, 100000.0);
range_float!(range_f64, f64, 123.456f64, 7890.12);

#[bench]
fn bernoulli_const(b: &mut Bencher) {
	let mut rand = urandom::new();
	b.iter(|| {
		let distr = distr::Bernoulli::new(0.18);
		let mut accum = true;
		for _ in 0..RAND_BENCH_N {
			accum ^= rand.sample(&distr);
		}
		accum
	})
}

#[bench]
fn bernoulli_var(b: &mut Bencher) {
	let mut rand = urandom::new();
	b.iter(|| {
		let mut accum = true;
		let mut p = black_box(0.18);
		for _ in 0..RAND_BENCH_N {
			let distr = distr::Bernoulli::new(p);
			accum ^= rand.sample(&distr);
			p += 0.0001;
		}
		accum
	})
}
