use ndarray::Array2;
use ndarray::Axis;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
#[macro_use(stack)]
extern crate ndarray;
fn main() {
    //println!("Hello, world!");
    let data = MatrixG { row: 1, col: 10 };
    let (_x, _b, _g, g_trans) = data.matrix_component();
    println!("g = {:#?}", g_trans);

}

#[derive(Debug)]
struct MatrixG {
    row: usize,
    col: usize,
}

impl MatrixG {
    fn matrix_component(
        &self,
    ) -> (Array2<f64>, Array2<f64>, Array2<f64>, Array2<f64>) {
        let x = Array2::random((self.row, self.col), Uniform::new(0., 10.));
        let b = Array2::<f64>::ones((self.row, self.col));
        let g = stack![Axis(1), b, x];
        let g_trans = stack![Axis(1), b, x].reversed_axes();
        (x, b, g, g_trans)
    }
}
