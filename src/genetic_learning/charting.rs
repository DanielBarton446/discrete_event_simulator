use plotters::prelude::*;

pub fn draw_data(
    chart_name: String,
    data: Vec<(f64, f64)>,
    dimensions: (u32, u32),
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let path = "target/tmp/".to_owned() + &chart_name;
    let root = BitMapBackend::new(&path, dimensions).into_drawing_area();
    root.fill(&WHITE)?;

    let minx = data
        .iter()
        .map(|(x, _)| *x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let maxx = data
        .iter()
        .map(|(x, _)| *x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let miny = data
        .iter()
        .map(|(_, y)| *y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let maxy = data
        .iter()
        .map(|(_, y)| *y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Wait Time By Evolution Count", ("serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(100)
        .build_cartesian_2d(minx..maxx, miny..maxy)?;

    chart
        .configure_mesh()
        .x_desc("Evolutions")
        .y_desc("Wait Time (seconds)")
        .draw()?;

    // ensure data is sorted by x value
    let mut data = data;
    data.sort_by(|(x1, _), (x2, _)| x1.partial_cmp(x2).unwrap());
    // Draw the line chart
    chart.draw_series(LineSeries::new(data, &RED))?;

    Ok(())
}
