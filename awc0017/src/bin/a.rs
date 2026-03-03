use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        cd: [(u64, u64); n],
    }
    let ans: u64 = cd.iter().filter(|(c, d)| *c <= k).map(|(c, d)| d).sum();
    println!("{}", ans);
}
