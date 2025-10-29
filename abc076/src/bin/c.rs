use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        mut s: Bytes,
        t: Bytes,
    }
    let mut ss = Vec::new();
    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }
    for i in 0..(s.len() - t.len() + 1) {
        let mut count = 0;
        for j in 0..t.len() {
            if s[i + j] == t[j] || s[i + j] == b'?' {
                count += 1;
            } else {
                count = 0;
                break;
            }
        }
        if count == t.len() {
            let mut res = String::new();
            for k in 0..s.len() {
                if k < i || i + t.len() <= k {
                    if s[k] == b'?' {
                        res.push('a');
                    } else {
                        res.push(s[k] as char);
                    }
                } else {
                    res.push(t[k - i] as char);
                }
            }
            ss.push(res);
        }
    }
    ss.sort();
    if ss.is_empty() {
        println!("UNRESTORABLE");
    } else {
        println!("{}", ss[0]);
    }
}
