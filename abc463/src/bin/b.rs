use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: char,
    }
    let mut ok = false;
    let idx = x as usize - 'A' as usize;
    for _ in 0..n {
        input! {
            s: Chars
        }

        if s[idx] == 'o' {
            ok = true;
        }
    }

    yes_no!(ok);
}
