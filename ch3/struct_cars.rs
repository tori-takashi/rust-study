struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}

fn main() {
    let car1 = CarSpec{
        model: 2018,
        cc: 2000,
        color: 1,
    };
    let car2 = CarSpec{
        model: 2019,
        cc: 2500,
        color: 2,
    };
    println!("{} {} {}", car1.model, car1.cc, car1.color);
    println!("{} {} {}", car2.model, car2.cc, car2.color);
}