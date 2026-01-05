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
    yes_no(a.abs_diff(c) <= d || (a.abs_diff(b) <= d && b.abs_diff(c) <= d));
}
