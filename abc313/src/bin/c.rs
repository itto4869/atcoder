use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u128; n],
    }
    a.sort_unstable();

    let sum = a.iter().sum::<u128>();

    let p = sum / n as u128;
    let r = sum % n as u128;

    let mut ans = 0;
    for i in 0..(n - r as usize) {
        ans += p.abs_diff(a[i]);
    }

    for i in (n - r as usize)..n {
        ans += (p + 1).abs_diff(a[i]);
    }

    println!("{}", ans / 2);
}
