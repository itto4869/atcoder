use std::collections::HashSet;

use cp_library::math::prime_enumeration::enumerate_primes;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut set = HashSet::new();
    for i in 0..(s.len() - 1) {
        for j in (i + 1)..s.len() {
            if s[i] == s[j] {
                set.insert((i, j));
            }
        }
    }

    let primes = enumerate_primes(10usize.pow(s.len() as u32) - 1);
    let min_v = 10usize.pow(s.len() as u32 - 1);
    for p in primes {
        if p < min_v {
            continue;
        }

        let mut ok = true;
        for i in (0..(s.len() - 1)) {
            for j in (i + 1)..s.len() {
                let a = p % 10usize.pow((s.len() - i) as u32);
                let b = p % 10usize.pow((s.len() - j) as u32);

                if set.contains(&(i, j)) && (a / (10usize.pow((s.len() - i - 1) as u32)) != b / (10usize.pow((s.len() - j - 1) as u32))) {
                    ok = false;
                    break;
                } else if !set.contains(&(i, j)) && (a / (10usize.pow((s.len() - i - 1) as u32)) == b / (10usize.pow((s.len() - j - 1) as u32))) {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            println!("{}", p);
            return;
        }
    }

    println!("-1");
}
