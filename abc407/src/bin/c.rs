use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut diff = 0;
    let mut ans = 0;
    for c in s.iter().rev() {
        let k = (c - b'0') as u64;
        let m = diff % 10;
        let curr = if m <= k {
            k - m
        } else {
            10 + k - m
        };

        diff += curr;
        ans += 1 + curr;
    }

    println!("{}", ans);
}
