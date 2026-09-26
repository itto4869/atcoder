use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut x = 1;
    let mut ok = false;
    for _ in 0..b {
        x = x * c;
        if x > a {
            ok = true;
            break;
        }
    }

    yes_no!(ok);
}
