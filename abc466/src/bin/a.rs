use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }
    yes_no!(x.iter().max().unwrap() < &0);
}
