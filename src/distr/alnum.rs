use super::*;

const ALPHANUMERIC: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Sample a `char`, uniformly distributed over ASCII letters and numbers: a-z, A-Z and 0-9.
///
/// # Examples
///
/// ```
/// use urandom::distr::Alnum;
/// let mut rand = urandom::new();
/// let chars: String = rand.samples(Alnum).take(7).collect();
/// println!("Random chars: {chars}");
/// ```
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Alnum;

impl Distribution<char> for Alnum {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rand: &mut Random<R>) -> char {
		loop {
			let val = rand.next_u32() >> (32 - 6);
			if (val as usize) < ALPHANUMERIC.len() {
				break ALPHANUMERIC[val as usize] as char;
			}
		}
	}
}
