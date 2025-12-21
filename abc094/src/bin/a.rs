use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    yes_no_custom(a <= x && a + b >= x, "YES", "NO");
}
