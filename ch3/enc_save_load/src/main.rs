use std::fs;
use std::fs::File;
use std::io::Write;
use encoding_rs;

fn main() {
    let filename = "text-sjis.txt";
    save_sjis(filename, "あいうえお");
    let s = load_sjis(filename);
    println!("{}", s);
}

fn save_sjis(filename: &str, s: &str) {
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(s);
    let buf = enc.into_owned();
    let mut file = File::create(filename).expect("作成");
    file.write(&buf[..]).expect("書き込み");
}

fn load_sjis(filename: &str) -> String {
    let buf = fs::read(filename).expect("読み込み");
    let (s, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    s.into_owned()
}
