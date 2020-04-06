pub fn f(x: f32) -> (f32, f32) {
    let polyordo5 = (63.0 * x.powi(5) - 70.0 * x.powi(3) + 15.0 * x) / 8.0;
    let polyordo4 = (35.0 * x.powi(4) - 30.0 * x.powi(2) + 3.0) / 8.0;
    (polyordo5, polyordo4)
}
