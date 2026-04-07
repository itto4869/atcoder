use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: u64,
        t: u64,
        mut d: [u64; n],
    }
    d.sort_unstable();

    let mut ok = true;
    for &di in &d {
        if s >= t {
            break;
        }
        if di <= s {
            s += di;
        } else {
            ok = false;
            break;
        }
    }

    if s < t {
        ok = false;
    }
    yes_no!(ok);
}
