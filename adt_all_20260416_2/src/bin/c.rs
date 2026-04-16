use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut b_idx = Vec::new();
    let mut k_idx = 0;
    let mut r_idx = Vec::new();
    for i in 0..s.len() {
        let c = s[i];
        if c == 'B' {
            b_idx.push(i);
        }

        if c == 'K' {
            k_idx = i;
        }

        if c == 'R' {
            r_idx.push(i);
        }
    }

    yes_no!((b_idx[0] % 2 != b_idx[1] % 2) && r_idx[0] < k_idx && k_idx < r_idx[1]);
}
