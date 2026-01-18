use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut sn = 0;
    let mut x = n;
    while x > 0 {
        sn += x % 10;
        x /= 10;
    }

    yes_no(n % sn == 0);
}
