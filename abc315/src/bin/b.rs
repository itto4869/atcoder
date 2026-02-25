use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        d: [u64; m],
    }
    let d_sum: u64 = d.iter().sum();
    let k = (d_sum + 1) / 2;
    let mut cnt = 0;
    for i in 0..m {
        cnt += d[i];
        if cnt >= k {
            println!("{} {}", i + 1, d[i] - (cnt - k));
            return;
        }
    }
}
