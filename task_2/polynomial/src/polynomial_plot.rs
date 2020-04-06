extern crate plotly;
use plotly::common::{DashType, Line, Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};

#[allow(dead_code)]
pub fn plot(x: Vec<f32>, res_x: Vec<f32>, res_y: Vec<f32>, ordo_5: &str, ordo_4: &str) {
    let trace1 = Scatter::new(x.clone(), res_x)
        .name(ordo_5)
        .mode(Mode::LinesMarkers)
        .line(Line::new().dash(DashType::DashDot));
    let trace2 = Scatter::new(x, res_y)
        .name(ordo_4)
        .mode(Mode::LinesMarkers)
        .line(Line::new().dash(DashType::DashDot));

    let layout = Layout::new()
        .title(Title::new("Polynomial"))
        .xaxis(
            Axis::new()
                .title(Title::new("X"))
                .show_grid(true)
                .zero_line(true),
        )
        .yaxis(Axis::new().title(Title::new("Y")).show_line(false));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);
    plot.show();
}
