use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = String::new();
    let mut idx = 0;
    while idx < n {
        if idx == (n - 1) {
            ans.push(s[idx]);
            idx += 1;
        } else {
            if s[idx] == 'n' && s[idx + 1] == 'a' {
                ans.push_str("nya");
                idx += 2;
            } else {
                ans.push(s[idx]);
                idx += 1;
            }
        }
    }

    println!("{}", ans);
}
