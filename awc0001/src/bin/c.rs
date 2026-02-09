use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut d: [u64; n],
    }
    d.sort_unstable();
    d.reverse();

    let mut ans = 0;
    for i in k..n {
        ans += d[i];
    }

    println!("{}", ans);
}
