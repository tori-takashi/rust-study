fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    echo("Hello, world!");
    echo("こんにちは、世界！");
    //let s = String::from("Hello, world!");
    //echo(&s);
}