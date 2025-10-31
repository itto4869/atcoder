use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut c1: Chars,
        c2: Chars,
    }
    c1.reverse();
    if c1 == c2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
