use core::mem::MaybeUninit;
use super::*;

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
		pub fn getentropy(buf: &mut [u8]) {
			getentropy_uninit(unsafe { util::slice_as_uninit_mut(buf) });
		}

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
		pub fn getentropy_uninit(buf: &mut [MaybeUninit<u8>]) {
			if let Err(_) = getrandom::getrandom_uninit(buf) {
				getentropy_not_ready()
			}
		}
	}
	else {
		extern "C" {
			fn getentropy_raw(ptr: *mut u8, len: usize) -> bool;
		}

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
		pub fn getentropy(buf: &mut [u8]) {
			getentropy_uninit(unsafe { util::slice_as_uninit_mut(buf) });
		}

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
		pub fn getentropy_uninit(buf: &mut [MaybeUninit<u8>]) {
			if buf.len() > 0 {
				if !unsafe { getentropy_raw(buf.as_mut_ptr() as *mut u8, buf.len()) } {
					getentropy_not_ready()
				}
			}
		}
	}
}

#[cold]
fn getentropy_not_ready() -> ! {
	panic!("getentropy not ready")
}
