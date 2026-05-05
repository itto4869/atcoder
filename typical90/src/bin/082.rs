use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;
const INV2: u64 = 500_000_004;

fn sum_range(l: u64, r: u64) -> u64 {
    // l + (l+1) + ... + r
    let a = l % MOD;
    let b = r % MOD;
    let n = (r - l + 1) % MOD;

    (((a + b) % MOD) * n % MOD) * INV2 % MOD
}

#[fastout]
fn main() {
    input! {
        l: u64,
        r: u64,
    }

    let start = l.ilog10() + 1;
    let end = r.ilog10() + 1;

    let mut ans = 0u64;

    for d in start..=end {
        let left = if d == start {
            l
        } else {
            10u64.pow(d - 1)
        };

        let right = if d == end {
            r
        } else {
            10u64.pow(d) - 1
        };

        let s = sum_range(left, right);
        ans += (d as u64) * s % MOD;
        ans %= MOD;
    }

    println!("{}", ans);
}