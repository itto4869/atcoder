use cp_utils::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut forward = vec![false; s.len() + 1];
    forward[0] = true;
    for i in 1..=t.len() {
        if s[i - 1] == '?' || t[i - 1] == '?' {
            forward[i] = forward[i - 1];
        } else {
            forward[i] = forward[i - 1] && (s[i - 1] == t[i - 1])
        }
    }

    let mut backward = vec![false; s.len() + 1];
    backward[s.len()] = true;
    for i in (0..t.len()).rev() {
        let j = i + s.len() - t.len();
        if s[j] == '?' || t[i] == '?' {
            backward[j] = backward[j + 1];
        } else {
            backward[j] = backward[j + 1] && (s[j] == t[i]);
        }
    }

    for i in 0..=t.len() {
        yes_no!(forward[i] && backward[s.len() - (t.len() - i)]);
    }
}
