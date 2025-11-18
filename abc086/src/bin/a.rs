use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    if (a * b) % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
