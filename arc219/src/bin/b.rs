use ac_library::{Max, Min, Segtree};
use proconio::{fastout, input};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; n],
        }
        if p[0] != 1 {
            println!("0");
            continue;
        }

        let mut ans = 0;
        for i in 0..n {
            if p[i] == (i + 1) {
                ans = (ans + n - i - 1) % MOD;
            } else {
                break;
            }
        }

        let mut ok = true;
        for i in 0..n {
            if p[i] != (i + 1) {
                ok = false;
                break;
            }
        }

        if ok {
            ans = (ans + 1) % MOD;
        }

        println!("{}", ans);
    }
}
