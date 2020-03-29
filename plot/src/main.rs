use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::LineStyle;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let f1 = Plot::from_function(|x| x.powi(3) * 5., 0., 10.)
        .line_style(LineStyle::new().colour("burlywood"));
    let f2 = Plot::from_function(|x| x.powi(2), 0., 10.)
        .line_style(LineStyle::new().colour("darkolivegreen").width(2.));
    let f3 = Plot::from_function(|x| x.sqrt() * 20., 0., 10.)
        .line_style(LineStyle::new().colour("brown").width(1.));

    let data = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
        (10.0, 4.3),
        (13.33, 3.8),
    ];

    let s: Plot = Plot::new(data).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .colour("#35C788"),
    );

    let v = ContinuousView::new().add(f1).add(f2).add(f3).add(s);

    Page::single(&v).save("test.svg").expect("saving svg");
}
