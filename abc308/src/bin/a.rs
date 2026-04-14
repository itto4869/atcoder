use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut ok = true;
    let mut pre_s = 0;

    for _ in 0..8 {
        input! {
            s: u64,
        }
        if s < pre_s {
            ok = false;
        }

        if s < 100 || s > 675 {
            ok = false;
        }

        if s % 25 != 0 {
            ok = false;
        }

        pre_s = s;
    }

    yes_no!(ok);
}
