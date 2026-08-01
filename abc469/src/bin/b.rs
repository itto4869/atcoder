use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..n {
        if s[i] == 'x' && (i == 0 || s[i - 1] == 'x') && (i == (n - 1) || s[i + 1] == 'x') {
            ans += 1;
        }
    }

    println!("{}", ans);
}
