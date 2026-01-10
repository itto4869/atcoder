use cp_library::math::numeric::GCD;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let ans = a.lcm(b);
    if ans > 10u128.pow(18) {
        println!("Large");
        return;
    }

    println!("{}", ans);
}
