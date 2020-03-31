use ndarray::Array2;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
mod plot_line;
use print;

fn main() {
    let x = Array2::random((1, 10), Uniform::new(0., 10.)).into_raw_vec();
    let y = Array2::random((1, 10), Uniform::new(0., 10.)).into_raw_vec();
    let _x_1 = Array2::random((1, 10), Uniform::new(0., 15.)).into_raw_vec();
    let _y_1 = Array2::random((1, 10), Uniform::new(0., 15.)).into_raw_vec();

    plot_line::scatter_plot(x, y, "Augi");
    //plot_line::line_dash(x_1, y_1, "Mas Kresna");
    print::print_this("Augi");
}
