use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let mut idx = 0;
    let mut ans = 0;
    while idx < (n - 1) {
        if a[idx + 1] - a[idx] <= k {
            ans += 1;
            idx += 2;
        } else {
            idx += 1;
        }
    }

    println!("{}", ans);
}
