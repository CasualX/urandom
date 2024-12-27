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
		pub fn getentropy<T: dataview::Pod>(buf: &mut [T]) -> &mut [T] {
			let buf: &mut [MaybeUninit<T>] = unsafe { mem::transmute(buf) };
			getentropy_uninit(buf)
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
		pub fn getentropy_uninit<T: dataview::Pod>(buf: &mut [MaybeUninit<T>]) -> &mut [T] {
			let dest = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut MaybeUninit<u8>, mem::size_of_val(buf)) };
			match getrandom::getrandom_uninit(dest) {
				Ok(_) => unsafe { mem::transmute(buf) },
				Err(_) => getentropy_not_ready(),
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
		pub fn getentropy<T: dataview::Pod>(buf: &mut [T]) -> &mut [T] {
			let buf: &mut [MaybeUninit<T>] = unsafe { mem::transmute(buf) };
			getentropy_uninit(buf)
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
		pub fn getentropy_uninit<T: dataview::Pod>(buf: &mut [MaybeUninit<T>]) -> &mut [T] {
			if buf.len() > 0 {
				if !unsafe { getentropy_raw(buf.as_mut_ptr() as *mut u8, mem::size_of_val(buf)) } {
					getentropy_not_ready()
				}
			}
			unsafe { mem::transmute(buf) }
		}
	}
}

#[cold]
fn getentropy_not_ready() -> ! {
	panic!("getentropy not ready")
}
