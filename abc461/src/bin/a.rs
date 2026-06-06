use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        d: usize,
    }
    yes_no!(a <= d);
}
