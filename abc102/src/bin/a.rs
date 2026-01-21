use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }
    yes_no_custom(n % 2 == 0, &n.to_string(), &(2 * n).to_string());
}
