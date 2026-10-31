use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
    }
    yes_no!(x * 9 == y * 16);
}
