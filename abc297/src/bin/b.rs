use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ok = true;
    let mut idx1 = 10;
    let mut idx2 = 10;
    let mut idx3 = 10;
    let mut idx4 = 10;
    let mut idx5 = 10;
    for i in 0..8 {
        if s[i] == 'B' {
            if idx1 == 10 {
                idx1 = i;
            } else {
                idx2 = i;
            }
        }

        if s[i] == 'K' {
            idx5 = i;
        }

        if s[i] == 'R' {
            if idx3 == 10 {
                idx3 = i;
            } else {
                idx4 = i;
            }
        }
    }

    if (idx1 % 2) == (idx2 % 2) {
        ok = false;
    }

    if !((idx3 < idx5) && (idx5 < idx4)) {
        ok = false;
    }

    yes_no!(ok);
}
