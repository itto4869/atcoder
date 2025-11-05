use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut x = n;
    let mut fx = 0;
    while x > 0 {
        fx += x % 10;
        x /= 10;
    }
    if n % fx == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
