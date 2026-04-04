use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    yes_no((a % 3 == 1 || a % 3 == 2) && b == a + 1);
}
