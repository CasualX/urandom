use crate::{Distribution, Random, Rng};
use crate::distributions::{UniformInt, UniformSampler};

/// Sample random elements of the slice with repetition.
#[derive(Copy, Clone, Debug)]
pub struct Choose<'a, T> {
	slice: &'a [T],
	range: UniformInt<usize>,
}
impl<'a, T> Choose<'a, T> {
	/// Construct a new `Choose` over the given slice.
	///
	/// # Panics
	///
	/// Panics if the slice is empty.
	#[inline]
	pub fn new(slice: &'a [T]) -> Choose<'a, T> {
		if slice.is_empty() {
			slice_is_empty();
		}
		let range = UniformInt::new(0, slice.len());
		Choose { slice, range }
	}
}
impl<'a, T> Distribution<&'a T> for Choose<'a, T> {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> &'a T {
		let index = self.range.sample(rng);
		&self.slice[index]
	}
}

#[cold]
fn slice_is_empty() -> ! {
	panic!("cannot sample from empty slice")
}

#[test]
fn test_yolo() {
	let mut rng = crate::new();
	let elements = [0; 100];
	// Test for various slice lengths
	for n in 1..100 {
		// Test for out of bounds panics
		let distr = Choose::new(&elements[..n]);
		for _ in 0..1000 {
			let _ = rng.sample(&distr);
		}
	}
}
