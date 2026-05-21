use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = Vec::with_capacity(2 * n);
    for i in 0..n {
        ans.push(s[i]);
        ans.push(s[i]);
    }
    println!("{}", ans.iter().collect::<String>());
}
