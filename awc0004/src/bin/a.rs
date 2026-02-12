use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: u64,
        t: u64,
        a: [u64; n],
    }
    let a_sum: u64 = a.iter().sum();
    yes_no(a_sum <= (t - s) * 60);
}
