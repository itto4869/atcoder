use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut w_sum = vec![0; n];
    w_sum[0] = if s[0] == b'W' {
        1
    } else {
        0
    };

    for i in 1..n {
        if s[i] == b'W' {
            w_sum[i] = w_sum[i - 1] + 1;
        } else {
            w_sum[i] = w_sum[i - 1];
        }
    }

    let mut ans = (n - 1) - (w_sum[n - 1] - w_sum[0]);
    for i in 1..n {
        let cnt = w_sum[i - 1] + ((n - i - 1) - (w_sum[n - 1] - w_sum[i]));
        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
