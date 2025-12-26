use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut ans = 700;
    for &c in &s {
        if c == b'o' {
            ans += 100;
        }
    }

    println!("{}", ans);
}
