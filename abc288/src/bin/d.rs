use cp_utils::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
    }
    let mut v = vec![vec![0; n + 1]; k];
    for j in 0..k {
        for i in 0..n {
            if (i % k) == j {
                v[j][i + 1] = v[j][i] + a[i];
            } else {
                v[j][i + 1] = v[j][i];
            }
        }
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: usize,
        }

        let mut ok = true;
        let val = v[0][r] - v[0][l];
        for j in 1..k {
            if val != (v[j][r] - v[j][l]) {
                ok = false;
                break;
            }
        }

        yes_no!(ok);
    }
}
