use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = false;
    for c in s {
        if c == 'x' {
            ok = false;
            break;
        } else if c == 'o' {
            ok = true;
        }
    }

    yes_no!(ok);
}
