fn main() {
    let args = std::env::args();
    let mut total = 0.0;
    for (i, s) in args.enumerate() {
        if i == 0 { continue; }
        let f : f64 = match s.parse() {
            Ok(n) => n,
            Err(_) => 0.0,
        };
        total += f;
    }
    println!("合計: {}", total);
    let args: Vec<String> = std::env::args().collect();
    println!("引数: {:?}", args)
}
