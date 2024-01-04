use std::collections::HashMap;

const V_DATA: &str = "C,C,C,A,A,B,C,C,C,A,C,B,C,C,C,C";

fn main() {
    let mut c_map: HashMap<&str, i32> = HashMap::new();
    c_map.insert("A", 0);
    c_map.insert("B", 0);
    c_map.insert("C", 0);
    for w in V_DATA.split(',') {
        c_map.insert(w, c_map[w]+1);
    }
    for key in ["A", "B", "C"] {
        println!("{}: {:>2}", key, c_map[key]);
    }
}

