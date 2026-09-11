use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = true;
    for i in 0..(n - 1) {
        if s[i] == s[i + 1] {
            ok = false;
            break;
        }
    }

    yes_no!(ok);
}
