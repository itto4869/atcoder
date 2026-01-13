use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    yes_no_custom(n < 1000, "ABC", "ABD");
}
