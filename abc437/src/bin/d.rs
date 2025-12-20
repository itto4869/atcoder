use proconio::{fastout, input};

const MOD: u128 = 998244353;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u128; n],
        mut b: [u128; m]
    }
    a.sort();
    b.sort();

    let mut a_sum = vec![0; n];
    a_sum[0] = a[0];
    for i in 1..n {
        a_sum[i] = a[i] + a_sum[i - 1];
    }

    let mut ans = 0;
    for &bi in &b {
        let idx = a.partition_point(|&x| x < bi);

        if idx > 0 && idx < n {
            ans = ((ans + a_sum[n - 1] - a_sum[idx - 1] - (n - idx) as u128 * bi) % MOD + (idx as u128 * bi - a_sum[idx - 1]) % MOD) % MOD;
        } else if idx == 0 {
            ans = (ans + (a_sum[n - 1] - n as u128 * bi) % MOD) % MOD;
        } else {
            ans = (ans + n as u128 * bi - a_sum[n - 1]) % MOD;
        }
    }

    ans = ans % MOD;
    println!("{}", ans);
}
