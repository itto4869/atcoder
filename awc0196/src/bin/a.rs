use cp_utils::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    yes_no!(s.contains(&'C') && s.contains(&'B'));
}
