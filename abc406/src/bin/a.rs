use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    yes_no(a > c || (a == c && b >= d));
}
