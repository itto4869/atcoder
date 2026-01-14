use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = String::new();
    for &c in &s {
        if c.is_uppercase() {
            ans.push(c);
        }
    }

    println!("{}", ans);
}
