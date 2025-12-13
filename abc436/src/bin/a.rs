use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut ans = String::new();
    for _ in 0..(n - s.chars().count()) {
        ans.push('o');
    }
    ans.push_str(&s);
    println!("{}", ans);
}
