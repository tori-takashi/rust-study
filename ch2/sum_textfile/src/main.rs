use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total: f64 = 0.0;
    for (i, filename) in args.enumerate() {
        if i == 0 {continue;}
        let text  = fs::read_to_string(filename).unwrap();
        let lines = text.split('\n');
        for line in lines { 
            let num: f64 = match line.parse() {
                Ok(n) => n,
                Err(_) => 0.0,
            };
            total += num;
        }
    }
    println!("合計: {}", total);
}
