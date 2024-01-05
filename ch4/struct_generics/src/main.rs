#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let pt_i = Point {x: 20, y: 20};
    let pt_f = Point {x: 20.0, y: 20.2};
    println!("{:?}", pt_i.x);
    println!("{:?}", pt_f.y);
}
