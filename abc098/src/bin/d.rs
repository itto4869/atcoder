use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = 0;
    let mut curr_sum = 0;
    let mut l = 0;
    for r in 0..n {
        while curr_sum & a[r] != 0 {
            curr_sum -= a[l];
            l += 1;
        }

        curr_sum += a[r];
        ans += r - l + 1;
    }

    println!("{}", ans);
}
