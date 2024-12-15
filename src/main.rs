mod data_loading;
mod statistics;
mod visualization;

use data_loading::Dataset;
use statistics::{pearson_correlation, linear_regression};
use visualization::{plot_correlation_graph, plot_heatmap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "stress_detection.csv";

    // Load dataset and compute averages
    let dataset = Dataset::load(file_path)?;
    let averages = dataset.compute_averages();

    let x_values: Vec<f64> = averages.iter().map(|avg| avg.pss).collect();
    let y_values: Vec<f64> = averages.iter().map(|avg| avg.psqi).collect();

    // Calculate statistics
    let correlation = pearson_correlation(&x_values, &y_values)?;
    println!("Pearson Correlation: {:.3}", correlation);

    let (slope, intercept) = linear_regression(&x_values, &y_values)?;
    println!("Linear Regression Model: Y = {:.3} + {:.3}X", intercept, slope);

    // Generate visualizations
    let regression_file = "correlation_graph_with_regression.png";
    plot_correlation_graph(&x_values, &y_values, slope, intercept, regression_file)?;
    println!("Scatter plot with regression line saved.");

    plot_heatmap(&x_values, &y_values, "heatmap_pss_psqi.png")?;
    println!("Heatmap saved.");

    Ok(())
}
