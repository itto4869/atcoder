use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        m: u32,
    }
    let ans = (100 * (n + 18 * m)) * 2u32.pow(m);
    println!("{}", ans);
}
