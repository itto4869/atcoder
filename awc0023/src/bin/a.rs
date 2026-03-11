use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        r: u64,
        t: [u64; n],
    }
    let ans = t.iter().sum::<u64>() + m * r;
    println!("{}", ans);
}
