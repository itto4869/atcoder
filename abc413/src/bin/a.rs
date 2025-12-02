use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    }
    let sum_a: u64 = a.iter().sum();
    if sum_a <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
