use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }
    yes_no(p <= x && x < (p + 100) && q <= y && y < (q + 100));
}
