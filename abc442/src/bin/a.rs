use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes
    }
    let mut ans = 0;
    for &c in &s {
        if c == b'i' || c == b'j' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
