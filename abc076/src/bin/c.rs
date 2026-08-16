use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut v = Vec::new();
    for i in 0..(s.len() - t.len() + 1) {
        let mut ok = true;
        for j in 0..t.len() {
            if (s[i + j] != '?') && (s[i + j] != t[j]) {
                ok = false;
                break;
            }
        }

        if ok {
            let mut ans = String::new();
            for k in 0..s.len() {
                if (k < i) || (k > (i + t.len() - 1)) {
                    if s[k] == '?' {
                        ans.push('a');
                    } else {
                        ans.push(s[k]);
                    }
                } else {
                    ans.push(t[k - i]);
                }
            }

            v.push(ans);
        }
    }

    v.sort_unstable();
    if v.is_empty() {
        println!("UNRESTORABLE");
    } else {
        println!("{}", v[0]);
    }
}
