use ac_library::{pow_mod, Mod998244353};
use proconio::{fastout, input};

const MOD: u64 = 998244353;
#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        p: [u64; n],
        w: [u64; d],
    }
    let sum_w = w.iter().sum::<u64>() % MOD;

    if w.iter().all(|&x| x == 0) {
        println!("0");
        return;
    }

    let mut s = 0;
    for &pi in &p {
        s = (s + pi) % MOD;
    }

    let mut prod_p = 1;
    for &pi in &p {
        prod_p = (prod_p * (pi % MOD)) % MOD;
    }

    let sum_w_n = pow_mod(sum_w as i64, n as i64, MOD as u32) as u64;

    let s_n = pow_mod(s as i64, n as i64, MOD as u32) as u64;

    let s_n_inv = pow_mod(s_n as i64, MOD as i64 - 2, MOD as u32) as u64;

    let ans = prod_p * sum_w_n % MOD * s_n_inv % MOD;

    println!("{}", ans);
}
