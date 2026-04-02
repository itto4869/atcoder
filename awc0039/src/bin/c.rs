use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
        b: [u64; n],
    }
    let mut a_sum = vec![0; n];
    let mut b_sum = vec![0; n];

    a_sum[0] = a[0];
    b_sum[0] = b[0];

    for i in 1..n {
        a_sum[i] = a_sum[i - 1] + a[i];
        b_sum[i] = b_sum[i - 1] + b[i];
    }

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    while r < n {
        if l == 0 {
            if b_sum[r] > k {
                l += 1;
            } else {
                ans = ans.max(a_sum[r]);
                r += 1;
            }
        } else {
            if b_sum[r] - b_sum[l - 1] > k {
                l += 1;
            } else {
                ans = ans.max(a_sum[r] - a_sum[l - 1]);
                r += 1;
            }
        }
    }

    println!("{}", ans);
}
