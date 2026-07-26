use core::{fmt, mem};
use super::*;

/// Rich interface for consuming random number generators.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Random<R: ?Sized> {
	#[cfg_attr(feature = "serde", serde(flatten))]
	rng: R,
}

impl<R> Random<R> {
	#[inline]
	pub(crate) const fn wrap(rng: R) -> Random<R> {
		Random { rng }
	}
}

impl<R: Rng + ?Sized> Random<R> {
	#[inline]
	pub(crate) fn next_u32(&mut self) -> u32 {
		self.rng.next_u32()
	}

	#[inline]
	pub(crate) fn next_u64(&mut self) -> u64 {
		self.rng.next_u64()
	}

	#[inline]
	pub(crate) fn next_f32(&mut self) -> f32 {
		self.rng.next_f32()
	}

	#[inline]
	pub(crate) fn next_f64(&mut self) -> f64 {
		self.rng.next_f64()
	}

	/// Fills the destination buffer with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Endianness
	///
	/// Bytes are written to `T`'s native in-memory representation.
	/// Multi-byte values are not portable across endianness.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let mut data = [0u32; 32];
	/// let data = rand.fill_bytes(&mut data);
	/// assert_ne!(data, [0u32; 32]);
	/// ```
	#[inline]
	pub fn fill_bytes<'a, T: dataview::Pod>(&mut self, buf: &'a mut [T]) -> &'a mut [T] {
		rng::util::fill_bytes(&mut self.rng, buf)
	}

	/// Fills the destination buffer with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Endianness
	///
	/// Bytes are written to `T`'s native in-memory representation.
	/// Multi-byte values are not portable across endianness.
	///
	/// # Examples
	///
	/// ```
	/// use std::mem::MaybeUninit;
	/// use std::slice;
	///
	/// let mut rand = urandom::new();
	/// let mut data = MaybeUninit::<[u32; 32]>::uninit();
	/// let data = rand.fill_bytes_uninit(slice::from_mut(&mut data));
	/// assert_ne!(data, [[0u32; 32]]);
	/// ```
	#[inline]
	pub fn fill_bytes_uninit<'a, T: dataview::Pod>(&mut self, buf: &'a mut [mem::MaybeUninit<T>]) -> &'a mut [T] {
		rng::util::fill_bytes_uninit(&mut self.rng, buf)
	}

	/// Fills the instance with uniform random bytes from the Rng.
	///
	/// The underlying Rng may implement this as efficiently as possible.
	///
	/// # Endianness
	///
	/// Bytes are written to `T`'s native in-memory representation.
	/// Multi-byte values are not portable across endianness.
	///
	/// # Examples
	///
	/// ```
	/// let mut rand = urandom::new();
	/// let value: [u32; 32] = rand.random_bytes();
	/// assert_ne!(value, [0u32; 32]);
	/// ```
	#[inline]
	pub fn random_bytes<T: dataview::Pod>(&mut self) -> T {
		rng::util::random_bytes(&mut self.rng)
	}

	/// Returns a clone of the current generator, then advances `self` by one jump.
	///
	/// Repeated calls produce deterministic, widely separated streams suitable for independent parallel computations.
	///
	/// # Panics
	///
	/// Panics if the underlying generator does not support jumping.
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

	/// Returns a sample from the [`StandardUniform`](distr::StandardUniform) distribution.
	///
	/// # Examples
	///
	/// ```
	/// let int: i8 = urandom::new().random();
	/// ```
	#[inline]
	pub fn random<T>(&mut self) -> T where distr::StandardUniform: Distribution<T> {
		distr::StandardUniform.sample(self)
	}

	/// Fills the given slice with samples from the [`StandardUniform`](distr::StandardUniform) distribution.
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
	pub fn fill<T>(&mut self, buf: &mut [T]) where distr::StandardUniform: Distribution<T> {
		let distr = distr::StandardUniform;
		for elem in buf {
			*elem = distr.sample(self);
		}
	}

	/// Returns a sample from the [`Uniform`](distr::Uniform) distribution within the given interval.
	///
	/// # Examples
	///
	/// ```
	/// let eyes = urandom::new().uniform(1..=6);
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
	#[track_caller]
	#[inline]
	pub fn uniform<T, I>(&mut self, interval: I) -> T where T: distr::SampleUniform, distr::Uniform<T>: From<I> {
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
	pub fn sample<T, D: Distribution<T>>(&mut self, distr: &D) -> T {
		distr.sample(self)
	}

	/// Returns an iterator of samples from the given distribution.
	///
	/// See the [`distr`] documentation for a list of available distributions.
	#[inline]
	pub fn samples<T, D: Distribution<T>>(&mut self, distr: D) -> distr::Samples<'_, R, D, T> {
		distr::Samples::new(self, distr)
	}

	/// Returns `true` with probability `p`.
	///
	/// This is known as the [`Bernoulli`](distr::Bernoulli) distribution.
	///
	/// # Precision
	///
	/// - For `p >= 1.0`, the resulting distribution will always generate `true`.
	/// - For `p <= 0.0`, the resulting distribution will always generate `false`.
	/// - For `p.is_nan()`, the resulting distribution will always generate `false`.
	#[inline]
	pub fn chance(&mut self, p: f64) -> bool {
		distr::Bernoulli::new(p).sample(self)
	}

	/// Flips a coin.
	///
	/// Returns `true` when heads and `false` when tails with 50% probability for either result.
	///
	/// Simply an alias for `rand.random::<bool>()` but describes the intent of the caller.
	#[inline]
	pub fn coin_flip(&mut self) -> bool {
		self.random()
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
	/// let fizzbuzz = rand.choose_iter((0..100).filter(is_fizzbuzz)).unwrap();
	/// assert!(fizzbuzz % 3 == 0 || fizzbuzz % 5 == 0);
	/// ```
	///
	/// Pick a random emoji:
	///
	/// ```
	/// let mood = urandom::new().choose_iter("😀😎😐😕😠😢".chars()).unwrap();
	/// println!("I am {mood}!");
	/// ```
	pub fn choose_iter<I: IntoIterator>(&mut self, collection: I) -> Option<I::Item> {
		let mut iter = collection.into_iter();

		// Take a short cut for collections with known length
		let (len, upper) = iter.size_hint();
		if upper == Some(len) {
			let index = usize::min(len, self.index(len));
			return iter.nth(index);
		}

		// Reservoir sampling optimized
		let mut result = None;
		for (i, item) in iter.enumerate() {
			if self.index(i + 1) == 0 {
				result = Some(item);
			}
		}
		result
	}

	/// Collect random samples from an iterator into the buffer.
	///
	/// The iterator is always exhausted, even when the buffer is already filled.
	/// This allows every item yielded by the iterator to participate in the random selection.
	///
	/// Although the elements are selected randomly, the order of elements in the buffer is neither stable nor fully random.
	/// If random ordering is desired, shuffle the result.
	///
	/// Returns the number of elements added to the buffer.
	/// This equals the length of the buffer unless the iterator yields insufficient
	/// elements, in which case it equals the number of yielded elements.
	///
	/// Complexity is `O(n)` where `n` is the size of the collection.
	pub fn choose_multiple<I: IntoIterator>(&mut self, collection: I, buf: &mut [I::Item]) -> usize {
		let amount = buf.len();
		let mut len = 0;

		for (i, elem) in collection.into_iter().enumerate() {
			if len < amount {
				buf[len] = elem;
				len += 1;
			}
			else {
				let k = self.index(i + 1);
				if let Some(slot) = buf.get_mut(k) {
					*slot = elem;
				}
			}
		}

		len
	}

	/// Returns a random usize in the `[0, len)` interval.
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
	/// 	assert!(index < len, "len:{len} index:{index} was not inbounds");
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

	/// Randomly selects up to _n_ elements and moves them into the start of the slice.
	///
	/// The selected elements are shuffled, and elements after the selected prefix may also be reordered by the selection process.
	/// If `n` is greater than or equal to the slice length, this shuffles the whole slice.
	///
	/// Returns the first _n_ shuffled elements and consumes _n_ values from the Rng.
	#[inline]
	pub fn partial_shuffle<'a, T>(&mut self, slice: &'a mut [T], n: usize) -> &'a mut [T] {
		let n = usize::min(n, slice.len());

		for i in 0..n {
			let k = self.uniform(i..slice.len());
			slice.swap(i, k);
		}

		&mut slice[..n]
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
	let mut rand = crate::new();

	let mut array = [0, 1, 2, 3, 4];
	let mut result = [0i32; 5];

	for _ in 0..10000 {
		result[*rand.choose(&array).unwrap()] += 1;
		result[*rand.choose_mut(&mut array).unwrap()] += 1;
	}

	let mean = (result[0] + result[1] + result[2] + result[3] + result[4]) / 5;
	let success = result.iter().all(|&x| (x - mean).abs() < 500);
	assert!(success, "mean: {mean}, result: {result:?}");
}

#[test]
fn test_choose_iter_reservoir() {
	let unknown_size = || (0..4).filter(|_| true);
	let mut rand = crate::new();
	let mut counts = [0i32; 4];

	for _ in 0..10000 {
		counts[rand.choose_iter(unknown_size()).unwrap()] += 1;
	}

	let mean = counts.iter().sum::<i32>() / counts.len() as i32;
	let success = counts.iter().all(|&count| (count - mean).abs() < 500);
	assert!(success, "mean: {mean}, counts: {counts:?}");

	assert_eq!(rand.choose_iter(core::iter::empty::<i32>()), None);
}

#[test]
fn test_choose_multiple_reservoir_range() {
	let mut rand = crate::new();
	let mut counts = [0i32; 2];

	for _ in 0..10000 {
		let mut result = [0];
		assert_eq!(rand.choose_multiple(0..2, &mut result), 1);
		counts[result[0]] += 1;
	}

	let mean = (counts[0] + counts[1]) / 2;
	let success = counts.iter().all(|&count| (count - mean).abs() < 500);
	assert!(success, "mean: {mean}, counts: {counts:?}");
}

#[test]
fn test_partial_shuffle() {
	let mut items = [1, 2, 3, 4, 100];
	let mut rng = crate::new();
	let items = rng.partial_shuffle(&mut items, 5);
	assert_eq!(items.len(), 5);
}
