fn main() {
    let height_cm = input("身長(cm): ");
    let weight_kg = input("体重(kg): ");
    let height = height_cm / 100.0;
    let bmi = weight_kg / height.powf(2.0);
    println!("あなたのBMIは{:.1}です。", bmi);

    if bmi < 18.5 {
        println!("あなたは低体重です。");
    } else if bmi < 25.0 {
        println!("あなたは普通体重です。");
    } else if bmi < 30.0 {
        println!("あなたは肥満(1度)です。");
    } else if bmi < 35.0 {
        println!("あなたは肥満(2度)です。");
    } else if bmi < 40.0 {
        println!("あなたは肥満(3度)です。");
    } else {
        println!("あなたは肥満(4度)です。");
    }
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    return s.trim().parse().expect("数値変換エラー");
}
