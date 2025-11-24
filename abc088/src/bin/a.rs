use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: u64,
    }
    if n % 500 <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}
