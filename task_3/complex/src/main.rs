mod complex;

fn main() {
    let x = complex::Complex::new(2.0, 3.0);
    let _mag = x.magnintude();
    let conj = x.conj();
    let y = complex::Complex::new(2.0, 1.0);
    let z = complex::Complex::div(x.clone(), y);
    x.view();
    conj.view();
    z.view();
}
