use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
    }
    yes_no_custom(400 % a == 0, &(400 / a).to_string(), "-1");
}
