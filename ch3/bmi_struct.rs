struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

fn main() {
    let height_cm = input("Height (cm): ");
    let weight_kg = input("Weight (kg): ");
    let height = height_cm / 100.0;
    let bmi = weight_kg / height.powf(2.0);
    let bmi_list = vec![
        BmiRange { min: 0.0, max: 18.5, label: "低体重" },
        BmiRange { min: 18.5, max: 25.0, label: "普通体重" },
        BmiRange { min: 25.0, max: 30.0, label: "肥満1度" },
        BmiRange { min: 30.0, max: 35.0, label: "肥満2度" },
        BmiRange { min: 35.0, max: 40.0, label: "肥満3度" },
        BmiRange { min: 40.0, max: std::f64::INFINITY, label: "肥満4度" },
    ];
    let mut result = "不明";
    for range in bmi_list {
        if range.min <= bmi && bmi < range.max {
            result = range.label;
            break;
        }
    }
    println!("あなたのBMIは{:.1}で{}です", bmi, result);
}

fn input(message: &str) -> f64 {
    println!("{}", message);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read line.");
    line.trim().parse().expect("Failed to parse.")
}