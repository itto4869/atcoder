use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    }
    yes_no!((a.is_negative() && b.is_negative()) || (a.is_positive() && b.is_positive()));
}
