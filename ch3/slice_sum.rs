fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for i in items {
        total += i;
    }
    total
}

fn main() {
    let a= [10, 20, 30, 40, 50];
    println!("{}", sum_slice(&a));
    let b = vec![1, 2, 3, 4, 5];
    println!("{}", sum_slice(&b[..]));
}