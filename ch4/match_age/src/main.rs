fn main() {
    let age = 18;
    let age_str = match age {
        0 => "newborn",
        1..=12 => "child",
        13..=19 => "teenager",
        20..=59 => "adult",
        _ => "old",
    };
    println!("age={}", age_str);
}
