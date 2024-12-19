use core::fmt;
use crate::*;

/// Rich interface for consuming random number generators.
#[derive(Clone)]
#[repr(transparent)]
pub struct Random<R: ?Sized> {
	rng: R,
}

impl<R> From<R> for Random<R> {
	#[inline]
	fn from(rng: R) -> Self {
		Random { rng }
	}
}

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
		self.rng.next_u32()
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
		self.rng.next_u64()
	}

	/// Returns a uniform random `f32` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 23 bits are necessary to construct a random float in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// If high quality uniform random floats are desired in open interval `(0.0, 1.0)` without bias see the [`Float01`](distr::Float01) distribution.
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_f32();
	/// assert!(value >= 1.0 && value < 2.0);
	/// ```
	#[inline]
	pub fn next_f32(&mut self) -> f32 {
		self.rng.next_f32()
	}

	/// Returns a uniform random `f64` in the half-open interval `[1.0, 2.0)`.
	///
	/// As only 52 bits are necessary to construct a random double in this range,
	/// implementations may override this method to provide a more efficient implementation.
	///
	/// If high quality uniform random floats are desired in open interval `(0.0, 1.0)` without bias see the [`Float01`](distr::Float01) distribution.
	///
	/// # Examples
	///
	/// ```
	/// let value = urandom::new().next_f64();
	/// assert!(value >= 1.0 && value < 2.0);
	/// ```
	#[inline]
	pub fn next_f64(&mut self) -> f64 {
		self.rng.next_f64()
	}

	/// Fills the destination buffer with random values from the Rng.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [0u32; 32];
	/// rand.fill_u32(&mut data);
	/// assert_ne!(data, [0; 32]);
	/// ```
	pub fn fill_u32(&mut self, mut buf: &mut [u32]) {
		while buf.len() >= 2 {
			let value = self.rng.next_u64().to_le_bytes();
			// Unaligned u64 little-endian write
			buf[0] = u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
			buf[1] = u32::from_le_bytes([value[4], value[5], value[6], value[7]]);
			buf = &mut buf[2..];
		}
		if buf.len() > 0 {
			buf[0] = self.rng.next_u32();
		}
	}

	/// Fills the destination buffer with uniform random values from the Rng.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [0u64; 32];
	/// rand.fill_u64(&mut data);
	/// assert_ne!(data, [0; 32]);
	/// ```
	pub fn fill_u64(&mut self, buf: &mut [u64]) {
		for elem in buf {
			*elem = self.rng.next_u64();
		}
	}

	/// Fills the destination buffer with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [0u8; 32];
	/// rand.fill_bytes(&mut data);
	/// assert_ne!(data, [0u8; 32]);
	/// ```
	#[inline]
	pub fn fill_bytes(&mut self, buf: &mut [u8]) {
		#![allow(unsafe_code)]
		self.rng.fill_bytes(unsafe { rng::util::slice_as_uninit_mut(buf) })
	}

	/// Fills the destination buffer with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [0u8; 32];
	/// rand.fill_bytes(&mut data);
	/// assert_ne!(data, [0u8; 32]);
	/// ```
	#[inline]
	pub fn fill_bytes_uninit(&mut self, buf: &mut [core::mem::MaybeUninit<u8>]) {
		self.rng.fill_bytes(buf)
	}

	/// Advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	#[inline]
	pub fn jump(&mut self) {
		self.rng.jump();
	}

	/// Clones the current instance and advances the internal state significantly.
	///
	/// Useful to produce deterministic independent random number generators for parallel computation.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// for _ in 0..10 {
	/// 	parallel_computation(rand.split());
	/// }
	/// # fn parallel_computation(_: urandom::Random<impl urandom::Rng>) {}
	/// ```
	#[inline]
	pub fn split(&mut self) -> Self where Self: Clone {
		let cur = self.clone();
		self.rng.jump();
		return cur;
	}

	/// Returns a sample from the [`Standard`](distr::Standard) distribution.
	///
	/// # Examples
	///
	/// ```
	/// let int: i8 = urandom::new().next();
	/// ```
	#[inline]
	pub fn next<T>(&mut self) -> T where distr::Standard: Distribution<T> {
		distr::Standard.sample(self)
	}

	/// Fills the given slice with samples from the [`Standard`](distr::Standard) distribution.
	///
	/// Because of its generic nature no optimizations are applied and all values are sampled individually.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [false; 32];
	/// rand.fill(&mut data);
	/// ```
	#[inline]
	pub fn fill<T>(&mut self, buf: &mut [T]) where distr::Standard: Distribution<T> {
		let distr = distr::Standard;
		for elem in buf {
			*elem = distr.sample(self);
		}
	}

	/// Returns a sample from the [`Uniform`](distr::Uniform) distribution within the given interval.
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
	/// let mut rand = urandom::new();
	/// let distr = urandom::distr::Uniform::from(0..100);
	///
	/// loop {
	/// 	let value = rand.sample(&distr);
	/// 	assert!(value >= 0 && value < 100);
	/// 	if value == 0 {
	/// 		break;
	/// 	}
	/// }
	/// ```
	#[inline]
	pub fn range<T, I>(&mut self, interval: I) -> T where T: distr::SampleUniform, distr::Uniform<T>: From<I> {
		distr::Uniform::<T>::from(interval).sample(self)
	}

	/// Returns a random float in the open `(0.0, 1.0)` interval.
	///
	/// This is a high quality uniform random float without bias in the low bits of the mantissa using the [`Float01`](distr::Float01) distribution.
	#[inline]
	pub fn float01(&mut self) -> f64 {
		distr::Float01.sample(self)
	}

	/// Returns a sample from the given distribution.
	///
	/// See the [`distr`] documentation for a list of available distributions.
	#[inline]
	pub fn sample<T, D>(&mut self, distr: &D) -> T where D: Distribution<T> {
		distr.sample(self)
	}

	/// Returns an iterator of samples from the given distribution.
	///
	/// See the [`distr`] documentation for a list of available distributions.
	#[inline]
	pub fn samples<T, D>(&mut self, distr: D) -> distr::Samples<'_, R, D, T> where D: Distribution<T> {
		distr::Samples::new(self, distr)
	}

	/// Returns `true` with the given probability.
	///
	/// This is known as the [`Bernoulli`](distr::Bernoulli) distribution.
	///
	/// # Precision
	///
	/// For `p >= 1.0`, the resulting distribution will always generate `true`.  
	/// For `p <= 0.0`, the resulting distribution will always generate `false`.  
	#[inline]
	pub fn chance(&mut self, p: f64) -> bool {
		distr::Bernoulli::new(p).sample(self)
	}

	/// Flips a coin.
	///
	/// Returns `true` when heads and `false` when tails with 50% probability for either result.
	///
	/// Simply an alias for `rand.next::<bool>()` but describes the intent of the caller.
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
	/// For slices, prefer [`choose`](Random::choose) which guarantees `O(1)` performance.
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
	/// let mut rand = urandom::new();
	/// let fizzbuzz = rand.single((0..100).filter(is_fizzbuzz)).unwrap();
	/// assert!(fizzbuzz % 3 == 0 || fizzbuzz % 5 == 0);
	/// ```
	///
	/// Pick a random emoji:
	///
	/// ```
	/// let mood = urandom::new().single("😀😎😐😕😠😢".chars()).unwrap();
	/// println!("I am {mood}!");
	/// ```
	pub fn single<I: IntoIterator>(&mut self, collection: I) -> Option<I::Item> {
		let mut iter = collection.into_iter();

		// Take a short cut for collections with known length
		let (len, upper) = iter.size_hint();
		if upper == Some(len) {
			let index = usize::min(len, self.index(len));
			return iter.nth(index);
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
	pub fn multiple<I: IntoIterator>(&mut self, collection: I, buf: &mut [I::Item]) -> usize {
		let amount = buf.len();
		let mut len = 0;

		collection.into_iter().enumerate().for_each(|(i, elem)| {
			if len < amount {
				buf[len] = elem;
				len += 1;
			}
			else {
				let k = self.index(i + 1 + amount);
				if let Some(slot) = buf.get_mut(k) {
					*slot = elem;
				}
			}
		});

		len
	}

	/// Returns a random usize in the `[0, len)` interval, mostly.
	///
	/// If the `len` is zero an arbitrary value is returned directly from the Rng.
	/// When used with indexing the bounds check should fail. Do not assume this value is inbounds.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// for len in 1..12345 {
	/// 	let index = rand.index(len);
	/// 	assert!(index < len, "len:{} index:{} was not inbounds", len, index);
	/// }
	/// ```
	pub fn index(&mut self, len: usize) -> usize {
		distr::UniformInt::constant(0, len).sample(self)
	}

	/// Returns a shared reference to one random element of the slice, or `None` if the slice is empty.
	#[inline]
	pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
		let index = self.index(slice.len());
		slice.get(index)
	}

	/// Returns a unique reference to one random element of the slice, or `None` if the slice is empty.
	#[inline]
	pub fn choose_mut<'a, T>(&mut self, slice: &'a mut [T]) -> Option<&'a mut T> {
		let index = self.index(slice.len());
		slice.get_mut(index)
	}

	/// Standard [Fisher–Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle) shuffle.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut array = [1, 2, 3, 4, 5];
	/// println!("Unshuffled: {array:?}");
	/// rand.shuffle(&mut array);
	/// println!("Shuffled:   {array:?}");
	/// ```
	#[inline]
	pub fn shuffle<T>(&mut self, slice: &mut [T]) {
		let mut len = slice.len();
		while len > 1 {
			let k = self.index(len);
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

#[cfg(feature = "std")]
impl<R: Rng + ?Sized> std::io::Read for Random<R> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.fill_bytes(buf);
		Ok(buf.len())
	}
	fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> std::io::Result<usize> {
		panic!("cannot read_to_end from Rng")
	}
	fn read_to_string(&mut self, _buf: &mut String) -> std::io::Result<usize> {
		panic!("cannot read_to_string from Rng")
	}
	fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
		self.fill_bytes(buf);
		Ok(())
	}
}

//----------------------------------------------------------------

#[test]
fn test_choose() {
	let mut rand = crate::new();

	let mut array = [0, 1, 2, 3, 4];
	let mut result = [0i32; 5];

	for _ in 0..10000 {
		result[*rand.choose(&array).unwrap()] += 1;
		result[*rand.choose_mut(&mut array).unwrap()] += 1;
	}

	let mean = (result[0] + result[1] + result[2] + result[3] + result[4]) / 5;
	let success = result.iter().all(|&x| (x - mean).abs() < 500);
	assert!(success, "mean: {}, result: {:?}", mean, result);
}
