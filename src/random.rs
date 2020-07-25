use core::fmt;
use crate::*;

/// Rich interface for consuming random number generators.
#[derive(Clone)]
pub struct Random<R: ?Sized>(pub R);

impl<R: Rng + ?Sized> Random<R> {
	/// Returns the next `u32` in the sequence.
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_u32();
	/// ```
	#[inline]
	pub fn next_u32(&mut self) -> u32 {
		self.0.next_u32()
	}

	/// Returns the next `u64` in the sequence.
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_u64();
	/// ```
	#[inline]
	pub fn next_u64(&mut self) -> u64 {
		self.0.next_u64()
	}

	/// Returns a uniform random `f32` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 23 bits are necessary to construct a random float in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// If high quality uniform random floats are desired in open interval `(0.0, 1.0)` without bias see the [`Float01`] distribution.
	///
	/// [`Float01`]: distributions/struct.Float01.html
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_f32();
	/// assert!(value >= 1.0 && value < 2.0);
	/// ```
	#[inline]
	pub fn next_f32(&mut self) -> f32 {
		self.0.next_f32()
	}

	/// Returns a uniform random `f64` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 52 bits are necessary to construct a random double in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// If high quality uniform random floats are desired in open interval `(0.0, 1.0)` without bias see the [`Float01`] distribution.
	///
	/// [`Float01`]: distributions/struct.Float01.html
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_f64();
	/// assert!(value >= 1.0 && value < 2.0);
	/// ```
	#[inline]
	pub fn next_f64(&mut self) -> f64 {
		self.0.next_f64()
	}

	/// Fills the destination buffer with random values from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible and may not be the same as simply filling with `next_u32`.
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let mut buffer = [0u32; 32];
	/// rng.fill_u32(&mut buffer);
	/// assert_ne!(buffer, [0; 32]);
	/// ```
	#[inline]
	pub fn fill_u32(&mut self, buffer: &mut [u32]) {
		self.0.fill_u32(buffer)
	}

	/// Fills the destination buffer with uniform random values from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible and may not be the same as simply filling with `next_u64`.
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let mut buffer = [0u64; 32];
	/// rng.fill_u64(&mut buffer);
	/// assert_ne!(buffer, [0; 32]);
	/// ```
	#[inline]
	pub fn fill_u64(&mut self, buffer: &mut [u64]) {
		self.0.fill_u64(buffer)
	}

	/// Fills the destination buffer with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let mut buffer = [0u8; 32];
	/// rng.fill_bytes(&mut buffer);
	/// assert_ne!(buffer, [0u8; 32]);
	/// ```
	#[inline]
	pub fn fill_bytes(&mut self, buffer: &mut [u8]) {
		self.0.fill_bytes(buffer)
	}

	/// Advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	#[inline]
	pub fn jump(&mut self) {
		self.0.jump();
	}

	/// Clones the current instance and advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// for _ in 0..10 {
	/// 	parallel_computation(rng.split());
	/// }
	/// # fn parallel_computation(_: urandom::Random<impl urandom::Rng>) {}
	/// ```
	#[inline]
	pub fn split(&mut self) -> Self where Self: Clone {
		let cur = self.clone();
		self.0.jump();
		return cur;
	}

	/// Returns a sample from the [`Standard`] distribution.
	///
	/// [`Standard`]: distributions/struct.Standard.html
	///
	/// # Examples
	///
	/// ```
	/// let int: i8 = urandom::new().next();
	/// ```
	#[inline]
	pub fn next<T>(&mut self) -> T where distributions::Standard: Distribution<T> {
		distributions::Standard.sample(self)
	}

	/// Fills the given slice with samples from the [`Standard`] distribution.
	///
	/// Because of its generic nature no optimizations are applied and all values are sampled individually from the distribution.
	///
	/// [`Standard`]: distributions/struct.Standard.html
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let mut buffer = [false; 32];
	/// rng.fill(&mut buffer);
	/// ```
	#[inline]
	pub fn fill<T>(&mut self, buffer: &mut [T]) where distributions::Standard: Distribution<T> {
		let distr = distributions::Standard;
		for elem in buffer {
			*elem = distr.sample(self);
		}
	}

	/// Returns a sample from the [`Uniform`] distribution within the given interval.
	///
	/// [`Uniform`]: distributions/struct.Uniform.html
	///
	/// # Examples
	///
	/// ```
	/// let eyes = urandom::new().range(1..=6);
	/// assert!(eyes >= 1 && eyes <= 6);
	/// ```
	///
	/// If more than one sample from a specific interval is desired, it is more efficient to reuse the uniform sampler.
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let distr = urandom::distributions::Uniform::from(..100);
	///
	/// loop {
	/// 	let value = rng.sample(&distr);
	/// 	assert!(value >= 0 && value < 100);
	/// 	if value == 0 {
	/// 		break;
	/// 	}
	/// }
	/// ```
	#[inline]
	pub fn range<T, I>(&mut self, interval: I) -> T where T: distributions::SampleUniform, distributions::Uniform<T>: From<I> {
		distributions::Uniform::<T>::from(interval).sample(self)
	}

	/// Returns a sample from the given distribution.
	///
	/// See the [`distributions`] documentation for a list of available distributions.
	///
	/// [`distributions`]: distributions/index.html
	#[inline]
	pub fn sample<T, D>(&mut self, distr: &D) -> T where D: Distribution<T> {
		distr.sample(self)
	}

	/// Returns an iterator of samples from the given distribution.
	///
	/// See the [`distributions`] documentation for a list of available distributions.
	///
	/// [`distributions`]: distributions/index.html
	#[inline]
	pub fn samples<T, D>(&mut self, distr: D) -> distributions::Samples<'_, R, D, T> where D: Distribution<T> {
		distributions::Samples::new(self, distr)
	}

	/// Returns `true` with the given probability.
	///
	/// This is known as the [`Bernoulli`] distribution.
	///
	/// [`Bernoulli`]: distributions/struct.Bernoulli.html
	///
	/// # Precision
	///
	/// For `p >= 1.0`, the resulting distribution will always generate `true`.  
	/// For `p <= 0.0`, the resulting distribution will always generate `false`.  
	#[inline]
	pub fn chance(&mut self, p: f64) -> bool {
		distributions::Bernoulli::new(p).sample(self)
	}

	/// Flips a coin.
	///
	/// Returns `true` when heads and `false` when tails with 50% probability for either result.
	///
	/// Simply an alias for `rng.next::<bool>()` but describes the intent of the caller.
	#[inline]
	pub fn coin_flip(&mut self) -> bool {
		self.next()
	}

	/// Returns a random sample from the collection.
	///
	/// Returns `None` if and only if the collection is empty.
	///
	/// This method uses `Iterator::size_hint` for optimisation.
	/// With an accurate hint and where `Iterator::nth` is a constant-time operation this method can offer `O(1)` performance.
	///
	/// For slices, prefer [`choose`] which guarantees `O(1)` performance.
	///
	/// [`choose`]: #method.choose
	///
	/// # Examples
	///
	/// Sample a random fizz, buzz or fizzbuzz number up to 100:
	///
	/// ```
	/// fn is_fizzbuzz(n: &i32) -> bool {
	/// 	n % 3 == 0 || n % 5 == 0
	/// }
	///
	/// let mut rng = urandom::new();
	/// let fizzbuzz = rng.single((0..100).filter(is_fizzbuzz)).unwrap();
	/// assert!(fizzbuzz % 3 == 0 || fizzbuzz % 5 == 0);
	/// ```
	///
	/// Pick a random emoji:
	///
	/// ```
	/// let mood = urandom::new().single("😀😎😐😕😠😢".chars()).unwrap();
	/// println!("I am {}!", mood);
	/// ```
	pub fn single<I: IntoIterator>(&mut self, collection: I) -> Option<I::Item> {
		let mut iter = collection.into_iter();

		// Take a short cut for collections with known length
		let (lower, upper) = iter.size_hint();
		if upper == Some(lower) {
			return if lower == 0 {
				None
			}
			else {
				let index = self.range(..lower);
				iter.nth(index)
			};
		}

		// Reservoir sampling, can be improved
		let mut result = None;
		let mut denom = 1.0;
		iter.for_each(|item| {
			if self.chance(1.0 / denom) {
				result = Some(item);
			}
			else {
				drop(item);
			}
			denom += 1.0;
		});
		result
	}
	/// Collect random samples from the collection into the buffer until it is filled.
	///
	/// Although the elements are selected randomly, the order of elements in the buffer is neither stable nor fully random.
	/// If random ordering is desired, shuffle the result.
	///
	/// Returns the number of elements added to the buffer.
	/// This equals the length of the buffer unless the iterator contains insufficient elements,
	/// in which case this equals the number of elements available.
	///
	/// Complexity is `O(n)` where `n` is the size of the collection.
	pub fn multiple<I: IntoIterator>(&mut self, collection: I, buffer: &mut [I::Item]) -> usize {
		let mut iter = collection.into_iter();

		let amount = buffer.len();
		let mut len = 0;
		while len < amount {
			if let Some(elem) = iter.next() {
				buffer[len] = elem;
				len += 1;
			}
			else {
				// Iterator exhausted; stop early
				return len;
			}
		}

		// Continue, since the iterator was not exhausted
		iter.enumerate().for_each(|(i, elem)| {
			let k = self.range(..i + 1 + amount);
			if let Some(slot) = buffer.get_mut(k) {
				*slot = elem;
			}
		});
		len
	}

	/// Returns a shared reference to one random element of the slice, or `None` if the slice is empty.
	#[inline]
	pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
		if slice.is_empty() {
			return None;
		}
		let index = self.range(..slice.len());
		slice.get(index)
	}
	/// Returns a unique reference to one random element of the slice, or `None` if the slice is empty.
	#[inline]
	pub fn choose_mut<'a, T>(&mut self, slice: &'a mut [T]) -> Option<&'a mut T> {
		if slice.is_empty() {
			return None;
		}
		let index = self.range(..slice.len());
		slice.get_mut(index)
	}

	/// Returns an iterator over random chosen elements of the slice with repetition.
	///
	/// # Panics
	///
	/// Panics if the slice is empty.
	#[inline]
	pub fn choose_iter<'a, T>(&mut self, slice: &'a [T]) -> distributions::Samples<'_, R, distributions::Choose<'a, T>, &'a T> {
		self.samples(distributions::Choose::new(slice))
	}

	/// Standard [Fisher–Yates] shuffle.
	///
	/// [Fisher–Yates]: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
	///
	/// # Examples
	///
	/// ```
	/// let mut rng = urandom::new();
	/// let mut array = [1, 2, 3, 4, 5];
	/// println!("Unshuffled: {:?}", array);
	/// rng.shuffle(&mut array);
	/// println!("Shuffled:   {:?}", array);
	/// ```
	#[inline]
	pub fn shuffle<T>(&mut self, slice: &mut [T]) {
		let mut len = slice.len();
		while len > 1 {
			let k = self.range(..len);
			slice.swap(k, len - 1);
			len -= 1;
		}
	}

	/// Shuffle only the first _n_ elements.
	///
	/// This is an efficient method to select _n_ elements at random from the slice without repetition, provided the slice may be mutated.
	#[inline]
	pub fn partial_shuffle<T>(&mut self, slice: &mut [T], mut n: usize) {
		if slice.len() > 1 {
			n = usize::min(n, slice.len() - 1);
			for i in 0..n {
				let k = self.range(i..slice.len());
				slice.swap(i, k);
			}
		}
	}
}

impl<R: Rng + ?Sized> fmt::Debug for Random<R> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Random(impl Rng)")
	}
}

//----------------------------------------------------------------

#[test]
fn test_choose() {
	let mut rng = crate::new();

	let mut array = [0, 1, 2, 3, 4];
	let mut result = [0i32; 5];

	for _ in 0..10000 {
		result[*rng.choose(&array).unwrap()] += 1;
		result[*rng.choose_mut(&mut array).unwrap()] += 1;
	}

	let mean = (result[0] + result[1] + result[2] + result[3] + result[4]) / 5;
	let success = result.iter().all(|&x| (x - mean).abs() < 500);
	assert!(success, "mean: {}, result: {:?}", mean, result);
}
