use cp_library::math::numeric::GCD;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            k: usize,
        }
        let x = d.lcm(n);
        let m = x / d;
        println!("{}", ((k - 1) * d + (k - 1) / m) % n);
    }
}
