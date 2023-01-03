use plotters::prelude::*;

pub(crate) fn draw_values(
    f_name: &str,
    input_name: &str,
    values: &Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("plotters-doc-data/{}", f_name);
    let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(input_name, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(100)
        // .margin(10)
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
        .x_labels(15)
        .y_labels(15)
        .y_desc("Hodnota stavu")
        .x_desc("Iterace")
        .axis_desc_style(("sans-serif", 20))
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
