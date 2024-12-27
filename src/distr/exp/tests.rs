use super::*;

#[test]
fn test_exp() {
	let exp = Exp::new(10.0);
	let mut rand = crate::new();
	for value in rand.samples(exp).take(1000) {
		eprintln!("{}", value);
		assert!(value >= 0.0);
	}
}

#[test]
fn test_zero() {
	let d = Exp::new(0.0);
	assert_eq!(crate::new().sample(&d), f64::INFINITY);
}

#[test]
#[should_panic]
fn test_exp_invalid_lambda_neg() {
	Exp::new(-10.0);
}

#[test]
#[should_panic]
fn test_exp_invalid_lambda_nan() {
	Exp::new(f64::NAN);
}

#[test]
fn exponential_distributions_can_be_compared() {
	assert_eq!(Exp::new(1.0), Exp::new(1.0));
}
