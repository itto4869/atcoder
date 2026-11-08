
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        p[i + 1] = p[i] + a[i];
    }

    let mut backward = vec![(0i64, 0i64); n + 1];
    backward[n] = (p[n], p[n]);

    for i in (0..n).rev() {
        backward[i] = (
            backward[i + 1].0.max(p[i]),
            backward[i + 1].1.min(p[i]),
        );
    }

    let mut forward = (0i64, 0i64);
    let mut ans = backward[0].0 - backward[0].1 + 1;

    for i in 0..n {
        let max_pos = forward.0.max(backward[i + 1].0 - 2 * a[i]);
        let min_pos = forward.1.min(backward[i + 1].1 - 2 * a[i]);

        ans = ans.max(max_pos - min_pos + 1);

        forward.0 = forward.0.max(p[i + 1]);
        forward.1 = forward.1.min(p[i + 1]);
    }

    println!("{}", ans);
}
