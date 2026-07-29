use super::*;

#[test]
fn samples_expected_medians() {
	for &(min, max, mode, median) in &[
		(-1.0, 1.0, 0.0, 0.0),
		(1.0, 2.0, 1.0, 2.0 - 0.5f64.sqrt()),
		(5.0, 25.0, 25.0, 5.0 + 200.0f64.sqrt()),
		(1e-5, 1e5, 1e-3, 1e5 - 4_999_999_949.5f64.sqrt()),
		(0.0, 1.0, 0.9, 0.45f64.sqrt()),
		(-4.0, -0.5, -2.0, -4.0 + 3.5f64.sqrt()),
	] {
		let mut rand = crate::rng::MockRng::repeat(0x8000_0000_0000_0000);
		assert_eq!(rand.random::<f64>(), 0.5);

		let mut rand = crate::rng::MockRng::repeat(0x8000_0000_0000_0000);
		assert_eq!(rand.sample(&Triangular::new(min, max, mode)), median);
	}
}

#[test]
fn validates_parameters() {
	assert_eq!(Triangular::try_new(2.0, 1.0, 1.0), Err(TriangularError::RangeTooSmall));
	assert_eq!(Triangular::try_new(f64::NAN, 1.0, 0.0), Err(TriangularError::RangeTooSmall));
	assert_eq!(Triangular::try_new(0.0, f64::NAN, 0.0), Err(TriangularError::RangeTooSmall));
	assert_eq!(Triangular::try_new(-1.0, 1.0, -2.0), Err(TriangularError::ModeRange));
	assert_eq!(Triangular::try_new(-1.0, 1.0, 2.0), Err(TriangularError::ModeRange));
	assert_eq!(Triangular::try_new(-1.0, 1.0, f64::NAN), Err(TriangularError::ModeRange));

	assert_eq!(Triangular::try_new(2.0f32, 1.0, 1.0), Err(TriangularError::RangeTooSmall));
	assert_eq!(Triangular::try_new(-1.0f32, 1.0, 2.0), Err(TriangularError::ModeRange));
}

#[test]
fn degenerate_distribution_is_exact() {
	let triangular = Triangular::new(-3.5, -3.5, -3.5);
	let mut rand = crate::rng::MockRng::repeat(u64::MAX);

	for _ in 0..10 {
		assert_eq!(rand.sample(&triangular), -3.5);
	}
}

#[test]
fn accessors_return_parameters() {
	let triangular = Triangular::new(-1.0f32, 4.0, 2.0);
	assert_eq!(triangular.min(), -1.0);
	assert_eq!(triangular.max(), 4.0);
	assert_eq!(triangular.mode(), 2.0);
}

#[test]
fn triangular_distributions_can_be_compared() {
	assert_eq!(Triangular::new(1.0, 3.0, 2.0), Triangular::new(1.0, 3.0, 2.0));
}

#[test]
#[should_panic]
fn invalid_new_panics() {
	Triangular::new(1.0, 0.0, 0.5);
}
