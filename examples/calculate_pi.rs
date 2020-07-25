
fn main() {
	let uniform = urandom::distributions::Uniform::from(-1.0f64..1.0f64);

	let mut rng = urandom::new();

	// The number of samples we'll take
	const N: usize = 1_000_000;

	let mut count = 0;
	for _ in 0..N {
		// Sample a point within the square with side 2.0
		let x = rng.sample(&uniform);
		let y = rng.sample(&uniform);

		// Count how many samples fall within the unit circle
		if (x * x + y * y).sqrt() < 1.0 {
			count += 1;
		}
	}

	// The area of a circle is `PI r²` and the radius in our example is `1`.
	// The area of a square is `side²` and the side in our example is `2`.
	// We know the ratio of these two is equal to our estimate.
	let ratio = count as f64 / (N as f64);

	// Scale the area of the square by this ratio to get `PI`.
	let pi = 4.0 * ratio;

	println!("PI is estimated as {}", pi);
}
