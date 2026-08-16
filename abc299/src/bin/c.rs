use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut cnt = 0;
    let mut ans = -1;
    for &c in &s {
        if c == 'o' {
            cnt += 1;
        } else {
            if cnt == 0 {
                continue;
            } else {
                ans = ans.max(cnt);
                cnt = 0;
            }
        }
    }

    cnt = 0;
    for &c in s.iter().rev() {
        if c == 'o' {
            cnt += 1;
        } else {
            if cnt == 0 {
                continue;
            } else {
                ans = ans.max(cnt);
                cnt = 0;
            }
        }
    }
    println!("{}", ans);
}
