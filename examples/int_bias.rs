fn main() {
	// Let's say our random number generator's range is `0..10`.
	// Let's say we want a random number in the interval `[0, 4)`.
	// To demonstrate the problem let's evaluate all generator outputs and see what number we get.

	println!("Random values in `[0, 4)` with bias:");
	for rng in 0..10 {
		// Using multiplication instead of remainder for performance reasons.
		// In practice the division is a power of two which is very efficient to work with.
		let value = rng * 4 / 10;

		print!("{}  ", value);
	}
	println!();

	// The result prints:
	// 0  0  0  1  1  2  2  2  3  3

	// Clearly showing that `0` and `2` are more likely to appear which isn't very uniform.
	// To fix this let's reject some randomness produced by the rng.

	println!("\nRandom values in `[0, 4)` without bias:");
	for rng in 0..10 {
		let value = rng * 4 / 10;
		// Here's the magic, for every value reject some which cause a biased result.
		// Again in practice the remainder is a power of two which is very efficient to work with.
		let reject = rng * 4 % 10;
		if reject >= 2 {
			print!("{}  ", value);
		}
	}
	println!();

	// The result prints:
	// 0  0  1  1  2  2  3  3

	// Now the distribution has been fixed, all the values in the requested interval are equally likely to appear.

	// Above the magic constant `2` was used to reject randomness.
	// This is very simple to calculate:
	// Observe that the range of the interval is `4` which fits in the underlying rng's range of `10` twice, with 2 to spare.
	// It's these two samples that we must reject as they are impossible to uniformly distribute over the input range.

	// This value can be calculated as `reject = (MAX - range) % range`.
	// For our example this is `reject = (10 - 4) % 4 = 2`.
	// In practice MAX will be `0` (does not fit in the integer) but wrapping_sub can be used.

	// This costs a potentially expensive integer division.
	// This can be worked around by using any other multiple of the range that is cheaper to calculate.
	// For example simply shift the range left until the most significant bit is 1.
	// It comes at the cost of potentially more rejections before a sample is accepted, but never more than 50%.
	// This can be avoided if the input range is known at compiletime where it can be constant folded.

	// Next is the calculation of the lookup table used for small ranges for u32 and u64 randomness ranges.
	if std::env::args().count() > 1 {
		println!("\nLookup table for u32 rejection sampling:");
		for range in 1..65 {
			let reject = u32::wrapping_sub(0, range) % range;
			print!("{}, ", reject);
			if range % 8 == 0 {
				println!();
			}
		}
		println!("\nLookup table for u64 rejection sampling:");
		for range in 1..65 {
			let reject = u64::wrapping_sub(0, range) % range;
			print!("{}, ", reject);
			if range % 8 == 0 {
				println!();
			}
		}
	}
}
