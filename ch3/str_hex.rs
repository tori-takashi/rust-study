fn main() {
    hex_dump("あいうえおおれはジャイアン");
}

fn hex_dump(s: &str) {
    for (i, c) in s.bytes().enumerate() {
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} " , c);
        }

        if i % 16 == 15 {
            println!("");
        }
    }
    println!("");
}