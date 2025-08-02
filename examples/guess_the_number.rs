
fn main() {
	println!("Guess the number between 0 and 100:");
	let secret = urandom::new().uniform(0..=100);

	let mut input = String::new();
	let mut tries = 0;
	loop {
		input.clear();
		std::io::stdin().read_line(&mut input).unwrap();

		let guess: i32 = match input.trim().parse() {
			Ok(guess) => guess,
			Err(_) => {
				println!("That was not a number! Try again.");
				continue;
			},
		};

		tries += 1;

		if guess == secret {
			println!("Congratulations! You guessed correctly after {tries} tries!");
			break;
		}
		else if guess < secret {
			println!("You guessed too low, try higher!");
		}
		else if guess > secret {
			println!("You guessed too high, try lower!");
		}
		else {
			unreachable!();
		}
	}
}
