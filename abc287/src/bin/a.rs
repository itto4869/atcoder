use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let cnt = s.into_iter().filter(|c| c == "For").count();
    yes_no!(cnt >= (n + 1) / 2);
}
