/*!
A quick refresher on IEEE 754 format for 32-bit floats;
when interpreted as an unsigned integers the bits are part of the following parts:

```
S_EEEEEEEE_MMMMMMMMMMMMMMMMMMMMMMM
```

Where S is the sign bit implementing one's complement, E are the 8 exponent bits and M are the 23 mantissa bits.

The half-open interval `[1.0, 2.0)` is represented by, S = `0`, E = `11111110` and M can be any random bits.
This is really easy to generate from the perspective of a random number generator.

It seems easy enough to just subtract 1.0 to get a floating point number in the interval `[0.0, 1.0)`.
But there is a problem which this example program demonstrates.
The low bits of the mantissa are less and less random as the random number gets lower.
This [loss of significance] is well documented with floating point addition and subtraction.

This can be worked around at a minimal cost in performance and is implemented by the Float01 distribution.

[loss of significance]: https://en.wikipedia.org/wiki/Loss_of_significance
*/

const N: usize = 10000;

fn main() {
	let mut rand = urandom::new();

	let mut buckets = [0u32; 8];
	for _ in 0..N {
		// Generate a random f32 in the interval `[0.0, 1.0)`
		let float = rand.next_f32() - 1.0;

		// Extract the low bits
		let bits = float.to_bits() & 0x7;

		// Keep track of the distribution of these low bits
		buckets[bits as usize] += 1;
	}
	print_buckets(&buckets, "naive implementation");

	let mut buckets = [0u32; 8];
	for _ in 0..N {
		// Generate a random f32 from the Float01 distribution.
		let float: f32 = rand.sample(&urandom::distr::Float01);

		// Extract the low bits
		let bits = float.to_bits() & 0x7;

		// Keep track of the distribution of these low bits
		buckets[bits as usize] += 1;
	}
	print_buckets(&buckets, "Float01 distribution");
}

fn print_buckets(buckets: &[u32], name: &str) {
	println!("\nThe low bits of {N} trials of the {name}:");
	println!("```");
	for (i, bucket) in buckets.iter().enumerate() {
		println!("{i:>#05b}: {bucket}");
	}
	println!("```");
}
