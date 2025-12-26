use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: i64,
        xv: [(i64, i64); n],
    }
    let mut cw = vec![0; n];
    let mut curr = 0;
    for i in 0..n {
        let (x, v) = xv[i];
        curr += v;
        cw[i] = curr - x;
    }

    let mut ccw = vec![0; n];
    curr = 0;
    for i in 0..n {
        let (x, v) = xv[n - 1 - i];
        curr += v;
        ccw[i] = curr - (c - x);
    }

    let mut cw_max = vec![0; n];
    let mut max_val = 0;
    for i in 0..n {
        max_val = max_val.max(cw[i]);
        cw_max[i] = max_val;
    }

    let mut ccw_max = vec![0; n];
    max_val = 0;
    for i in 0..n {
        max_val = max_val.max(ccw[i]);
        ccw_max[i] = max_val;
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(cw[i]);
        ans = ans.max(ccw[i]);
    }

    for i in 0..(n - 1) {
        let a_idx = i;
        let b_idx = n - 1 - i;

        let cw_return_val = cw[a_idx] - xv[a_idx].0;
        let combined = cw_return_val + ccw_max[b_idx - 1];

        ans = ans.max(combined);
    }

    for i in 0..(n - 1) {
        let b_idx = i;
        let a_idx = n - 1 - i;

        let (x, v) = xv[a_idx];
        let ccw_return_val = ccw[b_idx] - (c - x);
        let combined = ccw_return_val + cw_max[a_idx - 1];

        ans = ans.max(combined);
    }

    println!("{}", ans);
}
