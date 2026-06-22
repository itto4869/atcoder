use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

fn flip(c: char) -> char {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c.to_ascii_uppercase()
    }
}

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
        ks: [u64; q],
    }

    let n = s.len() as u64;

    let ans = ks
        .into_iter()
        .map(|k| {
            let k = k - 1; // 0-indexed
            let pos = (k % n) as usize;
            let block = k / n;

            let c = s[pos];

            if block.count_ones() % 2 == 0 {
                c
            } else {
                flip(c)
            }
        })
        .collect_vec();

    println!("{}", ans.iter().join(" "));
}