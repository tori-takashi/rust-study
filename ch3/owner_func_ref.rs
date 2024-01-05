fn main() {
    let g1 = String::from("Hello");
    show_message(&g1);
    println!("{}", g1);
}

fn show_message(msg: &str) {
    println!("{}", msg);
}