use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut ans = String::with_capacity(s.len());
    let mut ok = true;
    for &c in &s {
        if c == b'#' {
            ans.push(c as char);
            ok = true;
        } else if ok {
            ans.push('o');
            ok = false;
        } else {
            ans.push('.');
        }
    }
    println!("{}", ans);
}
