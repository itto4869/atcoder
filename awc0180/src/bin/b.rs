use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let mut ans = 1usize << 60;
    for i in 0..=(n - m) {
        let mut cnt = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                cnt += 1;
            }
        }

        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
