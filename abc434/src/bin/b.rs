use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut cnt = vec![(0u64, 0u64); m];
    for _ in 0..n {
        input! {
            a: Usize1,
            b: u64,
        }
        let (num, sum) = cnt[a];
        cnt[a] = (num + 1, sum + b);
    }

    for i in 0..m {
        let (num, sum) = cnt[i];
        let avg = (sum as f64) / (num as f64);
        println!("{}", avg);
    }
}
