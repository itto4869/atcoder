use cp_library::utils::yes_no;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    yes_no(s[0] == *s.last().unwrap());
}
