use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        n: u64,
    }
    let mut ans = 0;

    for length in 1..=12 {
        let half_len = (length + 1) / 2;

        let start = 10u64.pow(half_len - 1);
        let end = 10u64.pow(half_len);

        for i in start..end {
            let s = i.to_string();
            let rev_s: String = s.chars().rev().collect();

            let pal_str = if length % 2 == 0 {
                format!("{}{}", s, rev_s)
            } else {
                format!("{}{}", s, &rev_s[1..])
            };

            let val: u64 = pal_str.parse().unwrap();

            if val > n {
                break;
            }

            if is_palindrome_base_a(val, a) {
                ans += val;
            }
        }
    }

    println!("{}", ans);
}

fn is_palindrome_base_a(mut x: u64, a: u64) -> bool {
    let mut digits = Vec::with_capacity(64);
    while x > 0 {
        digits.push(x % a);
        x /= a;
    }

    digits.iter().eq(digits.iter().rev())
}