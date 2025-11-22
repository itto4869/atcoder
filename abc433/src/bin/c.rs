use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        mut s: Bytes,
    }
    let mut compress = Vec::new();
    let mut cnt = 1;
    for i in 0..(s.len() - 1) {
        if s[i] == s[i + 1] {
            cnt += 1;
        } else {
            compress.push((s[i] - b'0', cnt));
            cnt = 1;
        }
    }
    compress.push((s.last().unwrap() - b'0', cnt));
    let mut ans = 0;
    for i in 0..(compress.len() - 1) {
        let prev = compress[i];
        let next = compress[i + 1];
        if (prev.0 + 1) == next.0 {
            ans += prev.1.min(next.1);
        }
    }
    println!("{}", ans);
}
