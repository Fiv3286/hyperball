pub mod plotter {
    use plotters::{backend::BitMapBackend, 
        chart::ChartBuilder, 
        drawing::IntoDrawingArea, 
        element::Circle, 
        style::{full_palette::WHITE, RGBColor, ShapeStyle}};

    pub fn plot(xrange: std::ops::Range<u64>, yrange: std::ops::Range<f64>, points: Vec<(u64, f64)>) {
        let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(xrange, yrange) // adjusts schales; 
            .unwrap();

        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()
            .unwrap();

        chart
            .draw_series(
                points
                    .iter()
                    .map(|(x, y)| Circle::new((*x, *y), 5, Into::<ShapeStyle>::into(&RGBColor(0, 0, 255)))), 
            )
            .unwrap();
    }
}


