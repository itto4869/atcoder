use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        mut s: Bytes,
    }
    let mut idx = 0;
    let mut ans = 0;
    let mut flag = 0;
    while idx < s.len() {
        let c = s[idx];
        if (idx + 1) % 2 == flag && c == b'o' {
            idx += 1;
        } else if (idx + 1) % 2 == flag && c == b'i' {
            ans += 1;
            flag = 1 - flag;
        } else if (idx + 1) % 2 != flag && c == b'i' {
            idx += 1;
        } else {
            ans += 1;
            flag = 1 - flag;
        }
    }

    if (s.len() + ans) % 2 == 0 {
        println!("{}", ans);
    } else {
        println!("{}", ans + 1);
    }
}
