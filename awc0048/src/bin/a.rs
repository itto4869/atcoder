use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            s: String,
            k: u64,
        }
        yes_no!((s == "Yes" && k % 2 == 0) || (s == "No" && k % 2 == 1));
    }
}
