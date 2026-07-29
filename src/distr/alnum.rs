use super::*;

const ALNUM: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Uniform distribution over ASCII letters and numbers: a-z, A-Z and 0-9.
///
/// # Examples
///
/// ```
/// use urandom::distr::Alnum;
#[cfg_attr(feature = "getrandom", doc = "let mut rand = urandom::new();")]
#[cfg_attr(not(feature = "getrandom"), doc = "let mut rand = urandom::seeded(42);")]
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
			let value = rand.next_u32() >> (32 - 6);
			if (value as usize) < ALNUM.len() {
				break ALNUM[value as usize] as char;
			}
		}
	}
}
