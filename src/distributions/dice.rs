use crate::{Distribution, Random, Rng};
use crate::distributions::{UniformInt, UniformSampler};

/// Standard uniform dice.
///
/// # Examples
///
/// ```
/// use urandom::distributions::Dice;
/// let mut rng = urandom::new();
///
/// let sum: i32 = rng.samples(Dice::D6).take(2).sum();
/// assert!(sum >= 1 && sum <= 12);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Dice(UniformInt<u8>);

impl Dice {
	/// Constructs an N-sided dice.
	#[inline]
	pub fn new(n: u8) -> Dice {
		Dice(UniformInt::new_inclusive(1, n))
	}
}

impl Dice {
	/// 4-sided dice.
	///
	/// The Caltrop, always lands with the point face up. This dice is numbered 1-4.
	pub const D4: Dice = Dice(UniformInt::constant(1, 4, 0));
	/// 6-sided dice.
	///
	/// Is the standard cube-shaped dice, not only used in D&D, but different card and dice game as well.
	pub const D6: Dice = Dice(UniformInt::constant(1, 6, 4));
	/// 8-sided dice.
	///
	/// Is the eight-sided dice which used heavily for different strategies, at different points of gameplay.
	pub const D8: Dice = Dice(UniformInt::constant(1, 8, 0));
	/// 10-sided dice.
	///
	/// Used heavily, and a combination of two dice can result in moves 1 - 100.
	pub const D10: Dice = Dice(UniformInt::constant(1, 10, 6));
	/// 20-sided dice.
	///
	/// The signature dice of the dungeons and dragons game is the twenty sided dice.
	/// Is used most often in the game, and is the dice which is going to determine all of the strategies
	/// and attacks which will be used during game play by players. Also used to determine saving rolls during game play.
	pub const D20: Dice = Dice(UniformInt::constant(1, 20, 16));
}

impl Distribution<i32> for Dice {
	#[inline]
	fn sample<R: Rng + ?Sized>(&self, rng: &mut Random<R>) -> i32 {
		self.0.sample(rng) as i32
	}
}
