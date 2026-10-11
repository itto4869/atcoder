use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut cnt = 0;
    let mut ans = 0;
    for i in 0..n {
        if s[i] == 'R' {
            cnt += 1;
        } else {
            cnt = 0;
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
