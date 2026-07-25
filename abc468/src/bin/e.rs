use ac_library::{Mod998244353, ModInt, StaticModInt};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut imos = vec![0; n];
    imos[0] = a[0];
    for i in 1..n {
        imos[i] = imos[i - 1] + a[i];
    }

    let mut v: Vec<StaticModInt<Mod998244353>> = vec![StaticModInt::<Mod998244353>::new(0); n];
    for i in 0..((n + 1) / 2) {
        if i == 0 {
            v[i] = StaticModInt::<Mod998244353>::new(imos[n - 1]);
        } else {
            v[i] = v[i - 1] + StaticModInt::<Mod998244353>::new(imos[n - i - 1]) - StaticModInt::<Mod998244353>::new(imos[i - 1]);
        }
    }

    let mut ans = StaticModInt::<Mod998244353>::new(0);
    for d in 0..n {
        let idx = d.min(n - 1 - d);
        let p = v[idx];
        let q = StaticModInt::<Mod998244353>::new(d + 1);
        ans += p / q;
    }

    println!("{}", ans);
}
