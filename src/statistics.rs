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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pearson_correlation_valid_input() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = pearson_correlation(&x, &y).unwrap();
        assert!((result - 1.0).abs() < 1e-6, "Expected correlation close to 1.0");
    }

    #[test]
    fn test_linear_regression_valid_input() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope, intercept) = linear_regression(&x, &y).unwrap();
        assert!((slope - 2.0).abs() < 1e-6, "Expected slope of 2.0");
        assert!((intercept - 0.0).abs() < 1e-6, "Expected intercept of 0.0");
    }

    #[test]
    fn test_empty_input() {
        let x = vec![];
        let y = vec![];
        let result = pearson_correlation(&x, &y);
        assert!(result.is_err(), "Expected an error for empty input");
    }
}
