#![feature(test)]
extern crate test;

use urandom::rng::{SplittableRandom, Xoshiro256Rng};

// Batch enough operations that the benchmark measures the generators rather
// than Bencher's per-iteration bookkeeping. Reported times are per 1,000 forks.
const FORKS_PER_ITER: usize = 1_000;

macro_rules! fork_only {
	($name:ident, $generator:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let root = $generator;
			b.iter(|| {
				let mut current = root.clone();
				for _ in 0..FORKS_PER_ITER {
					let (left, right) = test::black_box(current).fork();
					test::black_box(right);
					current = left;
				}
				test::black_box(current)
			});
		}
	};
}

fork_only!(fork_only_xoshiro256, Xoshiro256Rng::from_seed_u64(42));
fork_only!(fork_only_splittable, SplittableRandom::from_seed_u64(42));

// A fork is normally followed by useful work. Consume the same number of u64s
// from both children to show how quickly generation cost amortizes fork cost.
macro_rules! fork_and_draw {
	($name:ident, $generator:expr, $draws:expr) => {
		#[bench]
		fn $name(b: &mut test::Bencher) {
			let root = $generator;
			b.iter(|| {
				let mut current = root.clone();
				let mut checksum = 0u64;
				for _ in 0..FORKS_PER_ITER {
					let (mut left, mut right) = test::black_box(current).fork();
					for _ in 0..$draws {
						checksum ^= left.random::<u64>();
						checksum ^= right.random::<u64>();
					}
					current = left;
				}
				test::black_box((current, checksum))
			});
		}
	};
}

fork_and_draw!(fork_draw_1_xoshiro256, Xoshiro256Rng::from_seed_u64(42), 1);
fork_and_draw!(fork_draw_1_splittable, SplittableRandom::from_seed_u64(42), 1);
fork_and_draw!(fork_draw_4_xoshiro256, Xoshiro256Rng::from_seed_u64(42), 4);
fork_and_draw!(fork_draw_4_splittable, SplittableRandom::from_seed_u64(42), 4);
fork_and_draw!(fork_draw_16_xoshiro256, Xoshiro256Rng::from_seed_u64(42), 16);
fork_and_draw!(fork_draw_16_splittable, SplittableRandom::from_seed_u64(42), 16);
fork_and_draw!(fork_draw_256_xoshiro256, Xoshiro256Rng::from_seed_u64(42), 256);
fork_and_draw!(fork_draw_256_splittable, SplittableRandom::from_seed_u64(42), 256);
