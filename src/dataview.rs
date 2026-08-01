//! Support for the optional external `dataview` API.
//!
//! This module is intentionally separate from the crate's internal `pod`
//! implementation. Dataview-backed operations use `dataview`'s own validity
//! guarantees and byte views.
#![allow(unsafe_code)]

use core::{mem, mem::MaybeUninit, slice};
use super::{Rng, Random};

/// Initializes a buffer so it can be exposed through `dataview::bytes_mut`.
#[inline]
pub fn initialize<T: ::dataview::Pod>(buf: &mut [MaybeUninit<T>]) -> &mut [T] {
	for value in &mut *buf {
		value.write(::dataview::zeroed());
	}
	// Every element was initialized above. MaybeUninit<T> has the same layout as T.
	unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<T>(), buf.len()) }
}

impl<R: Rng + ?Sized> Random<R> {
	/// Fills the destination plain-data buffer with uniform random bytes.
	///
	/// This is the typed counterpart to [`Random::fill_bytes`].
	/// Bytes are written to `T`'s native in-memory representation, so multi-byte values are not portable across endianness.
	#[inline]
	pub fn fill_data<'a, T: dataview::Pod>(&mut self, buf: &'a mut [T]) -> &'a mut [T] {
		self.fill_bytes(dataview::bytes_mut(buf));
		buf
	}

	/// Fills an uninitialized plain-data buffer with uniform random bytes.
	///
	/// This is the typed counterpart to [`Random::fill_bytes_uninit`].
	/// This method requires the `dataview` feature. Bytes are written to `T`'s
	/// native in-memory representation, so multi-byte values are not portable
	/// across endianness.
	#[inline]
	pub fn fill_data_uninit<'a, T: dataview::Pod>(&mut self, buf: &'a mut [mem::MaybeUninit<T>]) -> &'a mut [T] {
		let data = initialize(buf);
		self.fill_bytes(dataview::bytes_mut(data));
		data
	}

	/// Generates a plain-data value from uniform random bytes.
	///
	/// This method requires the `dataview` feature. Bytes are written to `T`'s
	/// native in-memory representation, so multi-byte values are not portable
	/// across endianness.
	#[inline]
	pub fn random_data<T: dataview::Pod>(&mut self) -> T {
		let mut value = dataview::zeroed();
		self.fill_bytes(dataview::bytes_mut(&mut value));
		value
	}
}

#[cfg(feature = "dataview")]
#[test]
fn test_data_api() {
	let expected = crate::seeded(42).random_bytes::<16>();

	let value: [u32; 4] = crate::seeded(42).random_data();
	assert_eq!(dataview::bytes(&value), expected);

	let mut rand = crate::seeded(42);
	let mut values = [0u32; 4];
	assert_eq!(rand.fill_data(&mut values).len(), 4);
	assert_eq!(dataview::bytes(&values), expected);

	let mut rand = crate::seeded(42);
	let mut values = [mem::MaybeUninit::<u32>::uninit(); 4];
	let values = rand.fill_data_uninit(&mut values);
	assert_eq!(dataview::bytes(values), expected);
}
