use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    s[0] = s[0].to_ascii_lowercase();
    let mut ans = "Of".to_string();
    let mut temp: String = s.iter().collect();
    ans.push_str(&temp);
    println!("{}", ans);
}
