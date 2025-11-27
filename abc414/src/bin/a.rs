use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: u64,
        r: u64,
        xy: [(u64, u64); n]
    }
    let ans = xy.iter().filter(|(a, b)| *a <= l && *b >= r ).count();
    println!("{}", ans);
}
