use cp_library::math::combinations::Combination;
use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let comb = Combination::new(n, MOD);
    for k in 1..=n {
        let mut ans = 0;
        for a in 1..=((n + k - 1) / k) {
            let d = n - (k - 1) * (a - 1);
            let cnt = comb.n_c_r(d, a);
            ans = (ans + cnt) % MOD;
        }

        println!("{}", ans);
    }
}
