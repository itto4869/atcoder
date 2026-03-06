use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }
    let sum: u64 = a.iter().sum();
    yes_no(sum % n == 0);
}
