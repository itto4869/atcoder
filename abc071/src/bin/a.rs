use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        a: u64,
        b: u64,
    }
    if x.abs_diff(a) < x.abs_diff(b) {
        println!("A");
    } else {
        println!("B");
    }
}
