use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }
    let mut i = 0;
    let mut j = 0;
    let mut ans = 0i64;
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else if s[i] == b'A' {
            i += 1;
            ans += 1;
        } else if t[j] == b'A' {
            j += 1;
            ans += 1;
        } else {
            ans = -1;
            break;
        }
    }

    if i == s.len() && j < t.len() {
        let mut ok = true;
        for idx in j..t.len() {
            let c = t[idx];
            if c != b'A' {
                ok = false;
                break;
            }
        }

        if ok {
            ans += (t.len() - j) as i64;
        } else {
            ans = -1;
        }
    } else if i < s.len() && j == t.len() {
        let mut ok = true;
        for idx in i..s.len() {
            let c = s[idx];
            if c != b'A' {
                ok = false;
                break;
            }
        }

        if ok {
            ans += (s.len() - i) as i64;
        } else {
            ans = -1;
        }
    }

    println!("{}", ans)
}
