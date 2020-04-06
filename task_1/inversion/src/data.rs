use ndarray::Array2;
use ndarray::Axis;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

#[derive(Debug)]
pub struct MatrixG {
    pub row: usize,
    pub col: usize,
}

impl MatrixG {
    pub fn matrix_component(&self) -> (Array2<f64>, Array2<f64>, Array2<f64>, Array2<f64>) {
        let x = Array2::random((self.row, self.col), Uniform::new(0., 10.));
        let b = Array2::<f64>::ones((self.row, self.col));
        let g = stack![Axis(1), b, x];
        let g_trans = stack![Axis(1), b, x].reversed_axes();
        (x, b, g, g_trans)
    }
}
