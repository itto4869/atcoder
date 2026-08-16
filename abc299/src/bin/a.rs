use cp_library::{yes_no, yes_no_custom};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = false;
    let mut f = 0;
    for c in s {
        if c == '|' {
            f += 1;
        } else if c == '*' {
            if f == 1 {
                ok = true;
            }
        }
    }

    yes_no_custom!(ok, "in", "out");
}
