use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: u64,
        x: u64,
    }
    if x == 1 {
        yes_no(1600 <= r && r <= 2999);
    } else {
        yes_no(1200 <= r && r <= 2399);
    }
}
