fn main() {
    let mut height;
    loop {
        println!("身長(cm)を入力してください");
        height = input_f(0.0);
        if height > 0.0 {
            break;
        }
        println!("正の数を入力してください");
    }
    let weight = 22.0 *(height / 100.0).powf(2.0);
    println!("身長{}cmの適正体重は{:.1}kgです。", height, weight);
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("入力エラー");
    s.trim().to_string()
}

fn input_f(default: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(n) => n,
        Err(_) => default,
    }
}
