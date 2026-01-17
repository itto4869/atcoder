use proconio::{fastout, input};

const MOD: i64 = 10i64.pow(9);
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut a = Vec::with_capacity(n + 1);
    let mut a_sum = vec![0; n + 1];
    for i in 0..=n {
        if i < k {
            a.push(1);
            a_sum[i] = i as i64 + 1;
        } else if i == k {
            a.push(k as i64);
            a_sum[i] = (k + k) as i64;
        } else {
            let mut ak = (a_sum[i - 1] - a_sum[i - k - 1]) % MOD;
            if ak < 0 {
                ak += MOD;
            }
            a_sum[i] = (ak + a_sum[i - 1]) % MOD;
            a.push(ak);
        }
    }
    
    println!("{}", a[n]);
}
