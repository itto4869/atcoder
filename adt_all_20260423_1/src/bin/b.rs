use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut ans = String::new();
    for i in 0..n {
        if i + 1 == (n + 1) / 2 {
            continue;
        } else {
            ans.push(s[i]);
        }
    }

    println!("{}", ans);
}
