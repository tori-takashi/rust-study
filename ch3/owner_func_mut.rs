fn add_quote(msg: &mut String) {
    msg.insert(0, '『');
    msg.push('』');
}

fn main() {
    let mut msg = String::from("Hello");
    println!("{}", msg);
    add_quote(&mut msg);
    println!("{}", msg);
}