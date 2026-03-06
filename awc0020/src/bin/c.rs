use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut v: [u64; n],
    }
    v.sort_unstable();
    let mut ans = 0;
    for i in 0..(n - 1) {
        ans += v[i].abs_diff(v[i + 1]);
    }

    println!("{}", ans);
}
