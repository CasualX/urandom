use super::*;

#[inline(always)]
pub fn rng_f32(seed: u32) -> f32 {
	f32::from_bits(127 << (f32::MANTISSA_DIGITS - 1) | (seed >> 9))
}

#[inline(always)]
pub fn rng_f64(seed: u64) -> f64 {
	f64::from_bits(1023 << (f64::MANTISSA_DIGITS - 1) | (seed >> 12))
}

#[inline(always)]
pub fn rng_fill_bytes<R: Rng>(rng: &mut R, buf: &mut [MaybeUninit<u8>]) {
	unsafe {
		let mut ptr = buf.as_mut_ptr() as *mut u8;
		let mut len = buf.len();

		// Loop unrolled for eight bytes at the time
		while len >= 8 {
			let value = rng.next_u64();
			// Unaligned u64 little-endian write
			ptr::copy_nonoverlapping(value.to_le_bytes().as_ptr(), ptr, 8);
			ptr = ptr.add(8);
			len -= 8;
		}

		if len > 0 {
			let mut value = rng.next_u64();

			if len >= 4 {
				// Unaligned u32 little-endian write
				ptr::copy_nonoverlapping((value as u32).to_le_bytes().as_ptr(), ptr, 4);
				ptr = ptr.add(4);
				len -= 4;
				value >>= 32;
			}

			if len >= 2 {
				// Unaligned u16 little-endian write
				ptr::copy_nonoverlapping((value as u16).to_le_bytes().as_ptr(), ptr, 2);
				ptr = ptr.add(2);
				len -= 2;
				value >>= 16;
			}

			if len >= 1 {
				ptr.write(value as u8);
			}
		}
	}
}

#[inline]
pub fn fill_bytes<'a, R: Rng + ?Sized>(rng: &mut R, buf: &'a mut [u8]) -> &'a mut [u8] {
	let dest = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<MaybeUninit<u8>>(), buf.len()) };
	rng.fill_bytes(dest);
	buf
}

#[inline]
pub fn fill_bytes_uninit<'a, R: Rng + ?Sized>(rng: &mut R, buf: &'a mut [MaybeUninit<u8>]) -> &'a mut [u8] {
	rng.fill_bytes(buf);
	unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<u8>(), buf.len()) }
}

#[inline]
pub fn random_bytes<R: Rng + ?Sized, const N: usize>(rng: &mut R) -> [u8; N] {
	let mut value = MaybeUninit::<[u8; N]>::uninit();
	fill_bytes_uninit(rng, value.as_mut());
	unsafe { value.assume_init() }
}
