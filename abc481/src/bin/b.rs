use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut prev = a[0];
    if prev == k {
        ans += 1;
    }
    for i in 1..n {
        if prev < a[i] {
            if prev < k && k <= a[i] {
                ans += 1;
            }
        } else {
            if prev > k && k >= a[i] {
                ans += 1;
            }
        }
        prev = a[i];
    }

    println!("{}", ans);
}
