use plotters::prelude::*;

pub(crate) fn draw_values(values: &Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/plot.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(
            0f32..values.len() as f32,
            *values.iter().min_by(|a, b| a.total_cmp(b)).unwrap()
                ..*values.iter().max_by(|a, b| a.total_cmp(b)).unwrap(),
        )?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        values
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i as f32, v.clone()))
            .collect::<Vec<(f32, f32)>>(),
        &RED,
    ))?;
    root.present()?;
    Ok(())
}
