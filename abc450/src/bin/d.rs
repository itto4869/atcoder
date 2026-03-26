use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    }
    let &max_a = a.iter().max().unwrap();
    for i in 0..n {
        let ai = a[i];
        let d = max_a - ai;
        let m = d / k;
        a[i] += k * m;
    }

    a.sort_unstable();
    let mut ans = max_a - a[0];
    for i in 0..(n - 1) {
        ans = ans.min(a[i] + k - a[i + 1]);
    }

    println!("{}", ans);
}
