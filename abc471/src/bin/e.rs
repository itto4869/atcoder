use ac_library::{Mod998244353, StaticModInt};
use proconio::{fastout, input};

type Mod998 = StaticModInt<Mod998244353>;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    if n == 1 {
        let ai = Mod998::new(a[0]);
        println!("{}", ai * ai);
        return;
    } else if k == 1 {
        let mut ans = Mod998::new(0);
        for &ai in &a {
            let ai = Mod998::new(ai);
            ans += ai * ai;
        }

        println!("{}", ans);
        return;
    }
    let mut imos = vec![0; n];
    imos[0] = a[0];
    for i in 1..n {
        imos[i] = imos[i - 1] + a[i];
    }

    let mut ans = Mod998::new(0);
    for &ai in &a {
        let ai = Mod998::new(ai);
        ans += ai * ai;
    }

    let mut x = Mod998::new(1);
    for i in (n - k + 1)..=(n - 1) {
        x *= Mod998::new(i);
    }

    for i in 1..=(k - 1) {
        x /= Mod998::new(i);
    }

    ans *= x;

    let mut y = Mod998::new(2);
    for i in (n - k + 1)..=(n - 2) {
        y *= Mod998::new(i);
    }

    for i in 1..=(k - 2) {
        y /= Mod998::new(i);
    }

    for i in 0..n {
        let l = Mod998::new(a[i]);
        let r = Mod998::new(imos[n - 1] - imos[i]);
        ans += y * l * r;
    }

    println!("{}", ans);
}
