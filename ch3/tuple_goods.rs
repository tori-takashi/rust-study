fn main() {
    let banana = ("バナナ", 300);
    let apple = ("りんご", 200);
    let total = banana.1 + apple.1;
    print_taple(&banana);
    print_taple(&apple);
    println!("合計 {}円", total);
}

fn print_taple(item: &(&str, i64)) {
    println!("{name}を{value}円で購入", name=item.0, value=item.1);
}