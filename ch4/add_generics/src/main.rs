fn add <T: std::ops::Add<Output=T>>( a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
    println!("{:.1}", add(1.0, 2.0));
}
