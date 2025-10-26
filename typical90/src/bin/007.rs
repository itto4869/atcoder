use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        q: usize,
        b: [u64; q],
    }
    a.sort();
    for &bi in &b {
        let idx = a.partition_point(|&x| x < bi).min(n - 1);
        let res = if a[idx].abs_diff(bi) > a[idx.saturating_sub(1)].abs_diff(bi) {
            a[idx.saturating_sub(1)].abs_diff(bi)
        } else {
            a[idx].abs_diff(bi)
        };
        println!("{}", res);
    }
}
