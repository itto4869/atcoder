use cp_library::{yes_no, yes_no_custom};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ok = 0i64;
    for c in s {
        if c == 'E' {
            ok += 1;
        } else {
            ok -= 1;
        }
    }

    yes_no_custom!(ok >= 0, "East", "West");
}
