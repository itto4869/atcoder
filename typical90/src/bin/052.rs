use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[u64; 6]; n],
    }
    let mut a_sum = vec![0; n];
    for i in 0..n {
        a_sum[i] = a[i].iter().sum();
    }

    let mut ans = 1;
    for &s in &a_sum {
        ans = (ans % MOD * s % MOD) % MOD;
    }

    println!("{}", ans);
}
