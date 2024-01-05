struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person{name, age}
    }
}

fn main() {
    let taro = Person::new("Taro".to_string(), 20);
    println!("name: {}, age: {}", taro.name, taro.age);
}
