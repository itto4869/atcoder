use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: u64,
    }
    yes_no_custom(200 <= s && s <= 299, "Success", "Failure");
}
