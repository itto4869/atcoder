use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let m = a / b;
    let c = a % b;
    yes_no_custom(c * 2 < b, &m.to_string(), &(m + 1).to_string());
}
