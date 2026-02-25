use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = String::new();
    for c in s {
        if c != 'a' && c != 'e' && c != 'i' && c != 'o' && c != 'u' {
            ans.push(c);
        }
    }

    println!("{}", ans);
}
