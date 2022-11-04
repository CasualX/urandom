use crate::Rng;

impl Rng for fn(&mut [u8]) {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		let mut value = 0;
		self(dataview::bytes_mut(&mut value));
		value
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		let mut value = 0;
		self(dataview::bytes_mut(&mut value));
		value
	}
	#[inline]
	fn fill_u32(&mut self, buffer: &mut [u32]) {
		self(dataview::bytes_mut(buffer))
	}
	#[inline]
	fn fill_u64(&mut self, buffer: &mut [u64]) {
		self(dataview::bytes_mut(buffer))
	}
	#[inline]
	fn fill_bytes(&mut self, buffer: &mut [u8]) {
		self(buffer)
	}
	#[inline]
	fn jump(&mut self) {
		// This method is intentionally left blank.
	}
}

cfg_if::cfg_if! {
	if #[cfg(feature = "getrandom")] {
		/// Provides cryptographically secure entropy.
		///
		/// # Panics
		///
		/// If unable to provide secure entropy this method will panic.
		///
		/// # Implementation notes
		///
		/// The implementation is provided by the [`getrandom`](https://crates.io/crates/getrandom) crate.
		#[inline]
		pub fn getentropy(buffer: &mut [u8]) {
			if let Err(_) = ::getrandom::getrandom(buffer) {
				getentropy_not_ready()
			}
		}
	}
	else {
		/// Provides cryptographically secure entropy.
		///
		/// # Panics
		///
		/// If unable to provide secure entropy this method will panic.
		///
		/// # Implementation notes
		///
		/// The implementation is provided by linking against an extern function.
		/// If `false` is returned then this function panics.
		///
		/// ```
		/// extern "C" {
		/// 	fn getentropy_raw(buffer_ptr: *mut u8, buffer_len: usize) -> bool;
		/// }
		/// ```
		#[inline]
		pub fn getentropy(buffer: &mut [u8]) {
			if !unsafe { getentropy_raw(buffer.as_mut_ptr(), buffer.len()) } {
				getentropy_not_ready()
			}
		}
		extern "C" {
			fn getentropy_raw(buffer_ptr: *mut u8, buffer_len: usize) -> bool;
		}
	}
}

#[cold]
fn getentropy_not_ready() -> ! {
	panic!("getentropy not ready")
}
