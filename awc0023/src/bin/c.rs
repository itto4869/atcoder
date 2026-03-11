use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        t: [u64; n],
    }
    let mut t_sum = vec![0; n + 1];
    for i in 0..n {
        t_sum[i + 1] = t_sum[i] + t[i];
    }

    for _ in 0..m {
        input! {
            s: u64,
            l: usize,
            r: usize,
        }
        let res = s + t_sum[r] - t_sum[l - 1];
        println!("{}", res);
    }
}
