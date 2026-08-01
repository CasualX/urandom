#![feature(test)]
extern crate test;

use urandom::{distr::*, Rng};

// Each reported iteration samples a batch of this size. Since all benchmarks
// use the same batch size, their reported times can be compared directly. The
// per-sample incremental overhead is:
//
//     (distribution ns/iter - matching Xoshiro256 ns/iter) / SAMPLES
//
// RNG and distribution construction deliberately happen outside the timed loop.
const SAMPLES: u64 = 10_000;

macro_rules! rng_bench {
	($name:ident, $output:ty, |$rand:ident| $sample:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut $rand = urandom::rng::Xoshiro256Rng::new();

			b.iter(|| {
				let mut accum: $output = 0;
				for _ in 0..SAMPLES {
					accum ^= $sample;
				}
				accum
			});
		}
	};
}

macro_rules! distribution_bench {
	($name:ident, $output:ty, $distribution:expr, |$rand:ident, $distr:ident| $sample:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let mut $rand = urandom::rng::Xoshiro256Rng::new();
			let $distr = test::black_box($distribution);

			b.iter(|| {
				let mut accum: $output = 0;
				for _ in 0..SAMPLES {
					accum ^= $sample;
				}
				accum
			});
		}
	};
}

// Xoshiro256 has specialized 32-bit integer and floating-point output paths.
rng_bench!(xoshiro256_next_u32, u32, |rand| rand.next_u32());
rng_bench!(xoshiro256_next_f32, u32, |rand| rand.next_f32().to_bits());

// UniformInt<u16> consumes next_u32, making next_u32 the matching baseline.
distribution_bench!(uniform_int_u16, u32, UniformInt::new(10u16, 1_000u16), |rand, distribution| rand.sample(&distribution) as u32);
distribution_bench!(uniform_float_f32, u32, UniformFloat::new(-10.0f32, 10.0f32), |rand, distribution| rand.sample(&distribution).to_bits());

rng_bench!(xoshiro256_next_u64, u64, |rand| rand.next_u64());
rng_bench!(xoshiro256_next_f64, u64, |rand| rand.next_f64().to_bits());

// These distributions consume next_u64 (Normal may consume more than one per sample), so next_u64 is their underlying-generator baseline.
distribution_bench!(uniform_int_u64, u64, UniformInt::new(10u64, 1_000_000u64), |rand, distribution| rand.sample(&distribution));
distribution_bench!(uniform_float_f64, u64, UniformFloat::new(-10.0f64, 10.0f64), |rand, distribution| rand.sample(&distribution).to_bits());
distribution_bench!(standard_normal_f32, u32, StandardNormal, |rand, distribution| rand.sample::<f32, _>(&distribution).to_bits());
distribution_bench!(normal_f32, u32, Normal::new(10.0f32, 2.0f32), |rand, distribution| rand.sample(&distribution).to_bits());
distribution_bench!(standard_normal_f64, u64, StandardNormal, |rand, distribution| rand.sample::<f64, _>(&distribution).to_bits());
distribution_bench!(normal_f64, u64, Normal::new(10.0f64, 2.0f64), |rand, distribution| rand.sample(&distribution).to_bits());
