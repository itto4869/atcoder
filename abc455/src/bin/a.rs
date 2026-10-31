use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    yes_no!((a != b) && (b == c));
}
