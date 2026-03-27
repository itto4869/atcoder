use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let k = a.iter().sum::<u64>();
    let mut a_sum = vec![0; n];
    a_sum[0] = a[0];
    for i in 1..n {
        a_sum[i] = a_sum[i - 1] + a[i];
    }

    let mut l = 0;
    let mut r = 0;
    let mut ok = false;
    while l < n {
        let m = if l > 0 && r < n {
            a_sum[r] - a_sum[l - 1]
        } else if l == 0 && r < n {
            a_sum[r]
        } else {
            a_sum[r % n] + (k - a_sum[l - 1])
        };

        if 10 * m == k {
            ok = true;
            break;
        } else if 10 * m > k {
            l += 1;
        } else {
            r += 1;
        }
    }

    yes_no(ok);
}
