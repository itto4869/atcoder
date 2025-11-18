use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u128,
        b: usize,
        c: u128,
    }
    let mut n = c;
    for _ in 1..b {
        n *= c;
        if n > a {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
