
#[inline]
pub fn rng_f32(seed: u32) -> f32 {
	f32::from_bits(0b0_01111111 << (f32::MANTISSA_DIGITS - 1) | (seed >> 9))
}
#[inline]
pub fn rng_f64(seed: u64) -> f64 {
	f64::from_bits(0b0_01111111111 << (f64::MANTISSA_DIGITS - 1) | (seed >> 12))
}
#[inline]
pub fn mantissa_f32(value: f32) -> u32 {
	value.to_bits() & ((1 << f32::MANTISSA_DIGITS - 1) - 1)
}
#[inline]
pub fn mantissa_f64(value: f64) -> u64 {
	value.to_bits() & ((1 << f64::MANTISSA_DIGITS - 1) - 1)
}

#[inline]
pub fn fill_u32<R: crate::Rng>(mut rng: R, mut buffer: &mut [u32]) -> R {
	while buffer.len() >= 2 {
		let value = rng.next_u64().to_le_bytes();
		// Unaligned u64 little-endian write
		buffer[0] = u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
		buffer[1] = u32::from_le_bytes([value[4], value[5], value[6], value[7]]);
		buffer = &mut buffer[2..];
	}
	if buffer.len() > 0 {
		buffer[0] = rng.next_u32();
	}
	rng
}
#[inline]
pub fn fill_u64<R: crate::Rng>(mut rng: R, buffer: &mut [u64]) -> R {
	for elem in buffer {
		*elem = rng.next_u64();
	}
	rng
}
#[inline]
pub fn fill_bytes<R: crate::Rng>(mut rng: R, mut buffer: &mut [u8]) -> R {
	// Loop unrolled for eight bytes at the time
	while buffer.len() >= 8 {
		let value = rng.next_u64();
		// Unaligned u64 little-endian write
		buffer[..8].copy_from_slice(&value.to_le_bytes());
		buffer = &mut buffer[8..];
	}
	if buffer.len() > 0 {
		let mut value = rng.next_u64();
		if buffer.len() >= 4 {
			// Unaligned u32 little-endian write
			buffer[0] = ((value >> 0) & 0xff) as u8;
			buffer[1] = ((value >> 8) & 0xff) as u8;
			buffer[2] = ((value >> 16) & 0xff) as u8;
			buffer[3] = ((value >> 24) & 0xff) as u8;
			buffer = &mut buffer[4..];
			value >>= 32;
		}
		if buffer.len() >= 2 {
			// Unaligned u16 little-endian write
			buffer[0] = ((value >> 0) & 0xff) as u8;
			buffer[1] = ((value >> 8) & 0xff) as u8;
			buffer = &mut buffer[2..];
			value >>= 16;
		}
		if buffer.len() >= 1 {
			buffer[0] = (value & 0xff) as u8;
		}
	}
	rng
}
