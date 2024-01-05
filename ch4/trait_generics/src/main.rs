fn x2<T: std::ops::Add<Output=T> + Copy> (n: T) -> T {
    n + n
}

fn main() {
    println!("{}", x2(1));
    println!("{}", x2(1.0));
    println!("{}", x2::<u64>(3));
    // println!("{}", x2("3"));
}
