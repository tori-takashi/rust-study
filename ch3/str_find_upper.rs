fn main() {
    let s = format! ("{}{}", 
        "Rust:", 
        "安全、速度、信頼性。");
    let res = s.find(|c: char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("{}B文字目に見つかりました。", i),
        None => println!("見つかりませんでした。"),
    }
    let s = s.replace("Rust", "C++");
    println!("{}", s);
}