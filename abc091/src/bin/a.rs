use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    if a + b >= c {
        println!("Yes");
    } else {
        println!("No");
    }
}
