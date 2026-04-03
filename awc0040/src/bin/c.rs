use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        mut x: [u64; n],
    }
    x.sort_unstable();
    let mut ans  = 0;
    for i in 0..n {
        let xi = x[i];
        let idx = x.partition_point(|&a| a < (xi.saturating_sub(k)));
        ans = ans.max(i - idx + 1);
    }

    println!("{}", ans);
}
