#[macro_use(stack)]
extern crate ndarray;
mod data;
use print;

fn main() {
    //println!("Hello, world!");
    let data = data::MatrixG { row: 1, col: 10 };
    let (_x, _b, _g, g_trans) = data.matrix_component();
    println!("g = {:?}", g_trans);
    print::print_this("Augi");
}
