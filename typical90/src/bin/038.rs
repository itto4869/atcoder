use cp_math::numeric::GCD;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let c = a.lcm(b);
    if c > 10u128.pow(18) {
        println!("Large");
    } else {
        println!("{}", c);
    }
}
