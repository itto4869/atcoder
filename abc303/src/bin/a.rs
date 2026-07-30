use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let map = |c: char| {
        if c == '0' {
            'o'
        } else if c == '1' {
            'l'
        } else {
            c
        }
    };
    
    let mut ok = true;
    for i in 0..n {
        let (c1, c2) = (map(s[i]), map(t[i]));
        if c1 != c2 {
            ok = false;
            break;
        }
    }

    yes_no!(ok);
}
