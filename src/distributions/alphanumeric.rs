use crate::{Distribution, Random, Rng};

const ALPHANUMERIC: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Sample a `char`, uniformly distributed over ASCII letters and numbers: a-z, A-Z and 0-9.
///
/// # Examples
///
/// ```
/// use urandom::distributions::Alphanumeric;
/// let mut rng = urandom::new();
/// let chars: String = rng.samples(Alphanumeric).take(7).collect();
/// println!("Random chars: {}", chars);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Alphanumeric;

impl Distribution<char> for Alphanumeric {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> char {
		loop {
			let val = rng.next_u32() >> (32 - 6);
			if (val as usize) < ALPHANUMERIC.len() {
				break ALPHANUMERIC[val as usize] as char;
			}
		}
	}
}
