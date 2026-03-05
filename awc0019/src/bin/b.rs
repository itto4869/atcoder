use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            s: Bytes,
        }
        let mut cnt = 0;
        for &c in &s {
            if c == b'!' {
                cnt += 1;
            }
        }

        if cnt >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
