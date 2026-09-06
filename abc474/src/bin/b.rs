use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut ok = true;
    for i in 1..=((n + 9) / 10) {
        for j in 0..10 {
            if ((i - 1) * 10 + j) >= n {
                break;
            }

            if !((10 * (i - 1) < p[(i - 1) * 10 + j]) && (p[(i - 1) * 10 + j] <= 10 * i)) {
                ok = false;
                break;
            }
        }
    }

    yes_no!(ok);
}
