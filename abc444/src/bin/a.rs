use cp_library::utils::yes_no;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: Bytes
    }
    yes_no(n[0] == n[1] && n[1] == n[2]);
}
