use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
        t: Bytes,
    }
    let mut ans = 0;
    for i in 0..n {
        if s[i] != t[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
