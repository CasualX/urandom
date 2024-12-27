use std::io;
use super::*;

/// Random number generator that reads random bytes from an [io::Read].
///
/// This will work best with an infinite reader, but that is not required.
///
/// # Panics
///
/// `Read` uses [`io::Read::read_exact`], which retries on interrupts.
/// All other errors from the underlying reader, including when it does not have enough data, will panic in case of an error.
///
/// # Examples
///
/// ```
/// let data = [1, 2, 3, 4, 5, 6, 7, 8];
/// let mut rand = urandom::rng::Read::new(&data[..]);
///
/// println!("{:x}", rand.next::<u32>());
/// ```
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Read<R> {
	reader: R,
}

impl<R> Read<R> {
	/// Creates a new instance.
	#[inline]
	pub fn new(reader: R) -> Random<Read<R>> {
		Random::wrap(Read { reader })
	}
}

impl<R> AsRef<R> for Read<R> {
	#[inline]
	fn as_ref(&self) -> &R {
		&self.reader
	}
}

impl<R> AsMut<R> for Read<R> {
	#[inline]
	fn as_mut(&mut self) -> &mut R {
		&mut self.reader
	}
}

impl<R: io::Read> Rng for Read<R> {
	#[inline]
	fn next_u32(&mut self) -> u32 {
		let mut buf = [0u8; 4];
		if let Err(err) = self.reader.read_exact(&mut buf) {
			read_failed(err);
		}
		u32::from_le_bytes(buf)
	}
	#[inline]
	fn next_u64(&mut self) -> u64 {
		let mut buf = [0u8; 8];
		if let Err(err) = self.reader.read_exact(&mut buf) {
			read_failed(err);
		}
		u64::from_le_bytes(buf)
	}
	#[inline]
	fn fill_bytes(&mut self, buf: &mut [MaybeUninit<u8>]) {
		let buf: &mut [u8] = unsafe { mem::transmute(buf) };
		if let Err(err) = self.reader.read_exact(buf) {
			read_failed(err);
		}
	}
	#[inline]
	fn jump(&mut self) {}
}

#[cold]
fn read_failed(err: io::Error) -> ! {
	panic!("random bytes from Read implementation failed: {:?}", err)
}

#[test]
fn test_next_u64() {
	// transmute from the target to avoid endianness concerns.
	let v = [
		0, 0, 0, 0, 0, 0, 0, 1,
		0, 4, 0, 0, 3, 0, 0, 2,
		5, 0, 0, 0, 0, 0, 0, 0u8];
	let mut rand = Read::new(&v[..]);

	assert_eq!(rand.next_u64(), 1 << 56);
	assert_eq!(rand.next_u64(), (2 << 56) + (3 << 32) + (4 << 8));
	assert_eq!(rand.next_u64(), 5);
}

#[test]
fn test_next_u32() {
	let v = [0u8, 0, 0, 1, 0, 0, 2, 0, 3, 0, 0, 0];
	let mut rand = Read::new(&v[..]);

	assert_eq!(rand.next_u32(), 1 << 24);
	assert_eq!(rand.next_u32(), 2 << 16);
	assert_eq!(rand.next_u32(), 3);
}

#[test]
fn test_fill_bytes() {
	let v = [1u8, 2, 3, 4, 5, 6, 7, 8];
	let mut w = [0u8; 8];

	let mut rand = Read::new(&v[..]);
	rand.fill_bytes(&mut w);

	assert!(v == w);
}

#[test]
#[should_panic(expected = "random bytes from Read implementation failed")]
fn test_insufficient_bytes() {
	let v = [1u8, 2, 3, 4, 5, 6, 7, 8];
	let mut w = [0u8; 9];

	let mut rand = Read::new(&v[..]);
	rand.fill_bytes(&mut w);
}
