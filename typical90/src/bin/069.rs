use ac_library::ModInt1000000007;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        k: u64,
    }
    if n == 1 && k < 1 {
        println!("0");
        return;
    }

    if n == 2 && k < 2 {
        println!("0");
        return;
    }

    if n >= 3 && k < 3 {
        println!("0");
        return;
    }

    if n == 1 {
        println!("{}", k);
        return;
    }

    if n == 2 {
        println!("{}", k * (k - 1));
        return;
    }

    let mut ans = ModInt1000000007::new(k * (k - 1));
    let mut exp = ModInt1000000007::new(k - 2);
    exp = exp.pow(n - 2);

    ans = ans * exp;
    println!("{}", ans);
}
