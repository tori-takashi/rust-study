struct Item(String, i64);
fn main() {
    let banana = Item("バナナ".to_string(), 300);
    let apple = Item("りんご".to_string(), 200);
    let mango = Item("マンゴー".to_string(), 500);
    let items = vec![banana, apple, mango];
    let total = print_and_sum_items(&items);
    println!("合計 {}円", total);
}

fn print_tuple(item: &Item) {
    println!("{name}を{value}円で購入", name=item.0, value=item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}