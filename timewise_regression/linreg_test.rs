use timewise_regression::linreg::LinReg;

#[test]
fn test_linreg_fit_predict() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.1, 6.0, 8.05, 10.1];
    let model = LinReg::fit(&x, &y).expect("fit failed");
    let pred = model.predict(6.0);
    assert!( (pred - 12.2).abs() < 0.5 );
    let mse = model.mse(&x, &y).unwrap();
    assert!(mse >= 0.0);
    let r2 = model.r2(&x, &y).unwrap();
    assert!(r2 > 0.95);
}