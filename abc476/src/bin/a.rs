use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    if *s.last().unwrap() == 'e' {
        s.push('r');
    } else {
        s.push('e');
        s.push('r');
    }

    let ans: String = s.into_iter().collect();
    println!("{}", ans);
}
