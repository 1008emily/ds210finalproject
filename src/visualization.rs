use plotters::prelude::*;

pub fn plot_correlation_graph(
    x: &Vec<f64>,
    y: &Vec<f64>,
    slope: f64,
    intercept: f64,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = *x.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let x_max = *x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_min = *y.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_max = *y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Correlation with Regression Line", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh()
        .x_desc("Average PSS Score")
        .y_desc("Average PSQI Score")
        .draw()?;

    chart.draw_series(
        x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| Circle::new((*xi, *yi), 5, BLUE.filled())),
    )?;

    let y_start = slope * x_min + intercept;
    let y_end = slope * x_max + intercept;

    chart.draw_series(LineSeries::new(
        vec![(x_min, y_start), (x_max, y_end)],
        &RED,
    ))?
    .label("Regression Line")
    .legend(|(x, y)| PathElement::new(vec![(x - 5, y), (x + 5, y)], &RED));

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    root.present()?;
    println!("Scatter plot with regression line saved to {}", output_file);

    Ok(())
}

pub fn plot_heatmap(
    x: &Vec<f64>,
    y: &Vec<f64>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = *x.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let x_max = *x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_min = *y.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let y_max = *y.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

    let grid_size = 20;
    let mut grid = vec![vec![0; grid_size]; grid_size];

    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let x_idx = ((xi - x_min) / (x_max - x_min) * (grid_size as f64 - 1.0)) as usize;
        let y_idx = ((yi - y_min) / (y_max - y_min) * (grid_size as f64 - 1.0)) as usize;
        grid[x_idx][y_idx] += 1;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption("Heatmap of PSS Score vs PSQI Score", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh()
        .x_desc("Average PSS Score")
        .y_desc("Average PSQI Score")
        .draw()?;

    let max_intensity = grid.iter().flatten().copied().max().unwrap_or(1) as f64;

    for i in 0..grid_size {
        for j in 0..grid_size {
            let x0 = x_min + i as f64 / grid_size as f64 * (x_max - x_min);
            let x1 = x_min + (i + 1) as f64 / grid_size as f64 * (x_max - x_min);
            let y0 = y_min + j as f64 / grid_size as f64 * (y_max - y_min);
            let y1 = y_min + (j + 1) as f64 / grid_size as f64 * (y_max - y_min);

            let intensity = grid[i][j] as f64 / max_intensity;

            let red = (255.0 * intensity * 0.7) as u8;   
            let green = (200.0 * (1.0 - intensity)) as u8; 
            let blue = (255.0 * (1.0 - intensity) * 0.7) as u8; 

            let color = RGBColor(red, green, blue);


            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x1, y1)],
                color.filled(),
            )))?;
        }
    }

    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_plot_correlation_graph() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let output_file = "test_correlation_graph.png";

        let result = plot_correlation_graph(&x, &y, 2.0, 0.0, output_file);
        assert!(result.is_ok(), "Expected function to run successfully");
        assert!(Path::new(output_file).exists(), "Expected output file to exist");

        std::fs::remove_file(output_file).unwrap();
    }
}
