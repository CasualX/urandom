use super::*;

#[test]
fn test_normal() {
	let norm = Normal::new(10.0, 10.0);
	let mut rand = crate::new();
	for value in rand.samples(norm).take(1000) {
		eprintln!("{}", value);
	}
}

#[test]
fn test_normal_cv() {
	let norm = Normal::from_mean_cv(1024.0, 1.0 / 256.0);
	assert_eq!((norm.mean, norm.std_dev), (1024.0, 4.0));
}

#[test]
fn test_normal_invalid_sd() {
	assert!(Normal::try_from_mean_cv(10.0, -1.0).is_err());
}

#[test]
fn test_log_normal() {
	let lnorm = LogNormal::new(10.0, 10.0);
	let mut rand = crate::new();
	for _ in 0..1000 {
		rand.sample(&lnorm);
	}
}

#[test]
fn test_log_normal_cv() {
	let lnorm = LogNormal::from_mean_cv(0.0, 0.0);
	assert_eq!(
		(lnorm.norm.mean, lnorm.norm.std_dev),
		(f64::NEG_INFINITY, 0.0)
	);

	let lnorm = LogNormal::from_mean_cv(1.0, 0.0);
	assert_eq!((lnorm.norm.mean, lnorm.norm.std_dev), (0.0, 0.0));

	let e = core::f64::consts::E;
	let lnorm = LogNormal::from_mean_cv(e.sqrt(), (e - 1.0).sqrt());
	assert!((lnorm.norm.mean - 0.0).abs() < 2e-16);
	assert!((lnorm.norm.std_dev - 1.0).abs() < 2e-16);

	let lnorm = LogNormal::from_mean_cv(e.powf(1.5), (e - 1.0).sqrt());
	assert!((lnorm.norm.mean - 1.0) < 1e-15);
	assert_eq!(lnorm.norm.std_dev, 1.0);
}

#[test]
fn test_log_normal_invalid_sd() {
	assert!(LogNormal::try_from_mean_cv(-1.0, 1.0).is_err());
	assert!(LogNormal::try_from_mean_cv(0.0, 1.0).is_err());
	assert!(LogNormal::try_from_mean_cv(1.0, -1.0).is_err());
}

#[test]
fn normal_distributions_can_be_compared() {
	assert_eq!(Normal::new(1.0, 2.0), Normal::new(1.0, 2.0));
}

#[test]
fn log_normal_distributions_can_be_compared() {
	assert_eq!(LogNormal::new(1.0, 2.0), LogNormal::new(1.0, 2.0));
}
