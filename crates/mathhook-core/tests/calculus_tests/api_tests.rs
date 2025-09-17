use mathhook_core::calculus::integrals::{
    AdaptiveSimpson, GaussianQuadrature, IntegrationConfig, NumericalIntegrator, RombergIntegration,
};

#[test]
fn test_public_api_accessibility() {
    let config = IntegrationConfig::default();

    // Test Gaussian quadrature
    let gaussian = GaussianQuadrature::new(5);
    let result = gaussian
        .integrate(|x: f64| x * x, 0.0, 1.0, &config)
        .unwrap();
    assert!((result.value - 1.0 / 3.0).abs() < 1e-6);

    // Test Simpson's rule
    let simpson = AdaptiveSimpson::new();
    let result = simpson
        .integrate(|x: f64| x * x, 0.0, 1.0, &config)
        .unwrap();
    assert!((result.value - 1.0 / 3.0).abs() < 1e-6);

    // Test Romberg integration
    let romberg = RombergIntegration::new(8);
    let result = romberg
        .integrate(|x: f64| x * x, 0.0, 1.0, &config)
        .unwrap();
    assert!((result.value - 1.0 / 3.0).abs() < 1e-6);
}
