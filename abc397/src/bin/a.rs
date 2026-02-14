use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: f64,
    }
    if x >= 38.0 {
        println!("1");
    } else if 37.5 <= x && x < 38.0 {
        println!("2");
    } else {
        println!("3");
    }
}
