use cp_library::yes_no_custom;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; k]; n],
    }
    let mut ok = true;
    for v in t.iter().multi_cartesian_product() {
        let mut x = 0;
        for ti in v {
            x ^= ti;
        }

        if x == 0 {
            ok = false;
            break;
        }
    }

    yes_no_custom!(ok, "Nothing", "Found");
}
