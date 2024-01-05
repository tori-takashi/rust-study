fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut n = 10;
    x2(&mut n);
    println!("{}", n);
}