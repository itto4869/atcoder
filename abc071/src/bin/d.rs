use proconio::{fastout, input, marker::Bytes};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s1: Bytes,
        s2: Bytes,
    }
    let mut pre_pattern = if s1[0] == s2[0] {
        0
    } else {
        1
    };
    let mut ans = if pre_pattern == 0 {
        3
    } else {
        6
    };
    let mut i = pre_pattern + 1;
    while i < n {
        let now_pattern: usize = if s1[i] == s2[i] {
            0
        } else {
            1
        };

        match (pre_pattern, now_pattern) {
            (0, 0) => {
                ans = ans * 2 % MOD;
                i += 1;
            },
            (0, 1) => {
                ans = ans * 2 % MOD;
                i += 2;
            },
            (1, 0) => {
                i += 1;
            },
            (1, 1) => {
                ans = ans * 3 % MOD;
                i += 2;
            },
            _ => unreachable!()
        }

        pre_pattern = now_pattern;
    }

    ans %= MOD;
    println!("{}", ans);

}
