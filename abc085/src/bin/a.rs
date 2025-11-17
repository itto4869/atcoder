use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let day = format!("{}{}", s[8] as char, s[9] as char);
    let mut ans = String::from("2018/01/");
    ans.push_str(&day);
    println!("{}", ans);
}
