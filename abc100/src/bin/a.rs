use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let d = a.abs_diff(b);
    yes_no_custom(a == b || 16 - a.min(b) * 2 > 2 * d + 1, "Yay!", ":(");
}
