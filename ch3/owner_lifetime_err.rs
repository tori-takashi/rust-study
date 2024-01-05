fn gen_message() -> String {
    let msg = String::from("Hello");
    msg
}

fn main() {
    let msg = gen_message();
    println!("{}", msg);
}