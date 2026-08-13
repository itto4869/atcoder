use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        n: u64,
    }

    let s: Vec<char> = s.into_iter().rev().collect();

    let mut ans = 0u64;

    for bit in 0..s.len() {
        if s[bit] == '1' {
            ans += 1u64 << bit;
        }
    }

    if ans > n {
        println!("-1");
        return;
    }

    for bit in (0..s.len()).rev() {
        if s[bit] == '?' {
            let x = 1u64 << bit;

            if ans + x <= n {
                ans += x;
            }
        }
    }

    println!("{}", ans);
}