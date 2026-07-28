use super::*;

#[test]
fn test_exp() {
	let mut rand = crate::new();
	let mut sum = 0.0f64;
	let samples = 100_000;
	for value in rand.samples::<f64, _>(Exp1).take(samples) {
		assert!(value.is_finite() && value > 0.0);
		sum += value;
	}
	let mean = sum / samples as f64;
	assert!((mean - 1.0).abs() < 0.02, "mean {mean}");
}

#[test]
fn test_zero() {
	let d = Exp::new(0.0);
	assert_eq!(crate::new().sample(&d), f64::INFINITY);
}

#[test]
fn test_infinite_rate_is_zero() {
	let d = Exp::new(f64::INFINITY);
	let mut rand = crate::new();
	for value in rand.samples(d).take(100) {
		assert_eq!(value, 0.0);
	}
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
fn test_exp_invalid_lambda_negative_zero() {
	assert_eq!(Exp::try_new(-0.0), Err(ExpError::LambdaTooSmall));
	assert_eq!(Exp::try_new(-0.0f32), Err(ExpError::LambdaTooSmall));
}

#[test]
fn exponential_distributions_can_be_compared() {
	assert_eq!(Exp::new(1.0), Exp::new(1.0));
}
