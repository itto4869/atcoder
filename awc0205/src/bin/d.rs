use ac_library::{Dsu, Mod998244353, StaticModInt};
use proconio::{fastout, input, marker::{Chars, Usize1}};

type Mod998 = StaticModInt<Mod998244353>;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        dsu.merge(u, v);
    }

    let mut ans = Mod998::new(1);
    for group in dsu.groups() {
        let mut zero_cnt = 0;
        let mut one_cnt = 0;

        for v in group {
            if s[v] == '0' {
                zero_cnt += 1;
            } else {
                one_cnt += 1;
            }
        }

        let mut cnt = Mod998::new(1);
        for x in 1..=(zero_cnt + one_cnt) {
            cnt *= x;
        }

        for x in 1..=(zero_cnt) {
            cnt /= x;
        }

        for x in 1..=(one_cnt) {
            cnt /= x;
        }

        ans *= cnt;
    }

    println!("{}", ans);
}
