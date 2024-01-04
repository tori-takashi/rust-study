use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dicfile = "ejdict-hand-utf8.txt";
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: jisyo <word>");
        std::process::exit(1);
    }

    let word = &args[1];

    let fp = File::open(dicfile).expect("failed to open file");
    let reader = BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.find(word) == None {continue;}
        println!("{}", line);
    }
}
