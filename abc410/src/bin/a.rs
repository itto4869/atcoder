use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
        k: u64,
    }
    let ans = a.iter().filter(|&&x| k <= x).count();
    println!("{}", ans);
}
