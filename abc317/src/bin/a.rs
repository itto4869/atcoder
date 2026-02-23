use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: u64,
        x: u64,
        p: [u64; n],
    }
    let d = x - h;
    let ans = p.partition_point(|&x| x < d) + 1;
    println!("{}", ans);
}
