use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 1..n {
        if s[i] == s[i - 1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
