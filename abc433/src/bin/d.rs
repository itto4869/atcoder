use std::collections::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
    }
    let mut ans = 0;
    let mut rem_count = vec![HashMap::new(); 11];

    for &ai in &a {
        let k = ai.ilog10() as usize + 1;
        let rem = ai % m;
        *rem_count[k].entry(rem).or_insert(0) += 1;
    }

    for k in 1..11 {
        if rem_count[k].is_empty() {
            continue;
        }

        let pow10_k = 10u64.pow(k as u32) % m;
        for &ai in &a {
            let lhs = (ai * pow10_k) % m;
            let target_rem = (m - lhs) % m;

            ans += rem_count[k].get(&target_rem).unwrap_or(&0);
        }
    }
    println!("{}", ans);
}
