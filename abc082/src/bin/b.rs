use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }
    s.sort();
    t.sort();
    t.reverse();
    let s: String = s.iter().collect();
    let t: String = t.iter().collect();
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
