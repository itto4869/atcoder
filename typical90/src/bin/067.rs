use proconio::{fastout, input};
use cp_library::math::base_conversion::convert_base;

#[fastout]
fn main() {
    input! {
        mut n: String,
        k: usize,
    }
    for _ in 0..k {
        n = convert_base(n, 8, 9).unwrap();
        n = n.replace("8", "5");
    }

    println!("{}", n);
}
