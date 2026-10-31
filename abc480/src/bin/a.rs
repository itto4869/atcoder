use cp_utils::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: [Chars; 2],
    }
    yes_no!((s[0][0] == 'A' && (s[0][1] == 'D' || s[1][0] == 'D'))
            || (s[0][1] == 'A' && (s[0][0] == 'D' || s[1][1] == 'D'))
            || (s[1][0] == 'A' && (s[0][0] == 'D' || s[1][1] == 'D'))
            || (s[1][1] == 'A' && (s[0][1] == 'D' || s[1][0] == 'D')))
}
