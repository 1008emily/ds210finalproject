pub fn pearson_correlation(x: &Vec<f64>, y: &Vec<f64>) -> Result<f64, &'static str> {
    if x.len() != y.len() || x.is_empty() {
        return Err("Vectors must have the same non-zero length");
    }

    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = x.iter().map(|xi| xi * xi).sum();
    let sum_y2: f64 = y.iter().map(|yi| yi * yi).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x.powi(2)) * (n * sum_y2 - sum_y.powi(2))).sqrt();

    Ok(numerator / denominator)
}

pub fn linear_regression(x: &Vec<f64>, y: &Vec<f64>) -> Result<(f64, f64), &'static str> {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let intercept = (sum_y - slope * sum_x) / n;

    Ok((slope, intercept))
}
