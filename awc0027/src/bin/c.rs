use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [u64; n],
    }
    x.sort_unstable();
    let mut ans = 0;
    for i in 1..n {
        ans = ans.max(x[i] - x[i - 1]);
    }

    println!("{}", ans);
}
