
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
