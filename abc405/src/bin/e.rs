use cp_library::math::combinations::Combination;
use proconio::{fastout, input};

const MOD: u64 = 998244353;
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let comb = Combination::new(a + b + c + d, MOD);

    let mut ans = 0;
    for i in 0..=c {
        ans = (ans + comb.n_c_r(a + b + i, b) * comb.n_c_r(d - 1 + c - i, d - 1)) % MOD;
    }

    println!("{}", ans);
}