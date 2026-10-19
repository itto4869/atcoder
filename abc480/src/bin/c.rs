use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let mut idx = 0;
    let mut curr = 0;
    let mut ans = 0;
    while curr < t && idx < n {
        if t <= a[idx] {
            ans += (t - curr + k - 1) / k;
            curr = a[idx] + k;
        } else if curr < a[idx] {
            ans += (a[idx] - curr + k - 1) / k;
            curr = a[idx] + k;
            idx += 1;
        } else {
            curr = curr.max(a[idx] + k);
            idx += 1;
        }
    }

    if curr < t {
        ans += (t - curr + k - 1) / k;
    }

    println!("{}", ans);
}
