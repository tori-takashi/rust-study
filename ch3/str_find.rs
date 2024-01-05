fn main() {
    let s = "隣の客はよく柿食う客だ";

    match s.find('柿') {
        Some(i) => println!("{}B文字目に「柿」が見つかりました。", i),
        None => println!("「柿」は見つかりませんでした。"),
    }

    match s.find('梨') {
        Some(i) => println!("{}文字目に「梨」が見つかりました。", i),
        None => println!("「梨」は見つかりませんでした。"),
    }
}