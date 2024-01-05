fn main() {
    let pr ="あいうえお";
    for c in pr.chars() {
        println!("[{}]", c);
    }
    println!("\n文字数: {}", pr.chars().count());
    let pr_chars: Vec<char> = pr.chars().collect();
    for c in pr_chars.iter() {
        println!("[{}]", c);
    }
    println!("\n文字数: {}", pr_chars.len());
}