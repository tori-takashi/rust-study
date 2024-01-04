fn encrypt(text: &str,  shift: i16) -> String {
    let a = 'A' as i16;
    let is_az = |c: char| 'A' <= c && c <= 'Z';
    let conv = |c: i16| ((((c)-a+shift+26)%26+a) as u8) as char;
    let enc1 = |c: char| if is_az(c) {conv(c as i16)} else {c};
    text.chars().map(|c: char| enc1(c)).collect::<String>()
}

fn main() {
    let enc = encrypt("I LOVE YOU", 3);
    let dec = encrypt(&enc, -3);
    println!("enc: {}, dec: {}", enc, dec);
}