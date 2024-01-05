fn main() {
    let s = "こんaにちは";
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    let s2 = &s[0..3];
    println!("{}", s2);
    let s3 = &s[0..7];
    println!("{}", s3);
}