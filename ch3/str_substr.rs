fn main() {
    let pr = "あいうえおおれはジャイアン";
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 { sub1.push(c); continue; }
        break;
    }
    println!("先頭2文字: {}", sub1);

    let sub2: String = pr.chars().take(2).collect();
    println!("先頭2文字: {}", sub2);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub3: &[char] = &pr_chars[8..13];
    let sub4: String = sub3.into_iter().collect();
    println!("8文字目から13文字目: {}", sub4);
}