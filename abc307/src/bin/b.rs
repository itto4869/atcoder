use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let ok = (0..n).any(|i| {
        (0..n).any(|j| {
            i != j && {
                let mut x = s[i].iter().chain(s[j].iter()).cloned().collect::<Vec<_>>();
                let y = x.clone();
                x.reverse();
                x == y
            }
        })
    });
    yes_no!(ok);
}
