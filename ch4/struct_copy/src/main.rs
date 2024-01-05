struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self{name: name.to_string(), age}
    }
}

fn main() {
    let taro = Person::new("Taro", 20);
    let jiro = Person{
        name: "Jiro".to_string(),
        ..taro
    };
    println!("name: {}, age: {}", taro.name, taro.age);
    println!("name: {}, age: {}", jiro.name, jiro.age);
}
