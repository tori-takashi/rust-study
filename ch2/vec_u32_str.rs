fn main() {
    let a_vec: Vec<u32> = vec![1, 2, 3, 4, 5, 10];
    println!("a_vec: {:?}", a_vec);
    let a_str: Vec<&str> = vec!["one", "two", "three"];
    println!("a_str: {:?}", a_str);
}