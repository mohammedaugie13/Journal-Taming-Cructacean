mod point;
fn main() {
    let p1 = point::Point::new(3.9, 4.3);
    let p2 = point::Point::new(3.2, 4.3);
    let x = p1.add(p2.clone());
    let x_1 = p2.clone().sub(p1.clone());
    let x_2 = p2.clone().mul(p1.clone());
    let x_3 = point::Point::dot(p1.clone(), p2.clone());
    println!("{:?}", x);
    println!("{:?}", x_1);
    println!("{:?}", x_2);
    println!("{:?}", x_3);
}
