use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoch.as_millis() as u32;
    }
    SEED ^= SEED << 2;
    SEED ^= SEED >> 5;
    SEED ^= SEED << 5;
    SEED % (end - start + 1) + start
}

fn main() {
    let mut count: HashMap<u32, i32> =  [1, 2, 3, 4, 5, 6].iter().map(|x| (*x, 0)).collect();

    for _ in 0..10000 {
        unsafe {
            let v = rand_global(1, 6);
            count.insert(v as u32, count[&v] + 1);
        }
    }

    for (k, v) in count.iter() {
        println!("{}: {}回", k, v);
    }
}
