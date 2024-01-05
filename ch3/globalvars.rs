static mut TAX: f32 = 0.1;

fn main() {
    unsafe {
        println!("Price: {}", 100.0 * (1.0 + TAX));
        TAX = 0.08;
        println!("Price: {}", 100.0 * (1.0 + TAX));
    }
}