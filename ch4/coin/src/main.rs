enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}

impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(n) => n,
            Coin::Coin5(n) => n * 5,
            Coin::Coin10(n) => n * 10,
            Coin::Coin50(n) => n * 50,
            Coin::Coin100(n) => n * 100,
            Coin::Coin500(n) => n * 500,
        }
    }
}

fn main() {
    let wallet: Vec<Coin> = vec![
        Coin::Coin500(3),
        Coin::Coin100(2),
        Coin::Coin50(1),
        Coin::Coin10(3),
        Coin::Coin5(2),
        Coin::Coin1(4),
    ];
    let total = wallet.iter().fold(0, |sum, v| sum + v.calc_price());
    println!("total={}", total);
}
