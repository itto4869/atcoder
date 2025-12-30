use cp_library::utils::yes_no_custom;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    
    yes_no_custom(a <= b, &a.to_string(), &(a - 1).to_string());
}
