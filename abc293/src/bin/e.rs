use proconio::{fastout, input};
use ac_library::ModInt as Mint;

// (a^n, 1 + a + ... + a^(n-1)) を返す
fn calc(a: Mint, n: u64) -> (Mint, Mint) {
    if n == 0 {
        return (Mint::new(1), Mint::new(0));
    }

    let (p, s) = calc(a, n / 2);

    // p = a^k, s = S(k)
    let p2 = p * p;
    let s2 = s * (Mint::new(1) + p);

    if n % 2 == 0 {
        (p2, s2)
    } else {
        (p2 * a, s2 + p2)
    }
}

#[fastout]
fn main() {
    input! {
        a: u64,
        x: u64,
        m: u32,
    }

    Mint::set_modulus(m);
    let (_, ans) = calc(Mint::new(a), x);

    println!("{}", ans);
}