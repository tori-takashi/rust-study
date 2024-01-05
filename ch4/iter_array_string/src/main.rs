fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
    ];
    for a in &array {
        println!("{}", a);
    }
    println!("{:?}", array)
}
