use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        md: (u64, u64)
    }
    yes_no(md == (1, 7) || md == (3, 3) || md == (5, 5) || md == (7, 7) || md == (9, 9));
}
