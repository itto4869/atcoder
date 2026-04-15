use proconio::input;
use std::collections::HashMap;

const MOD: u64 = 1_000_000_007;

fn enumerate_half(arr: &[u64], m: u64) -> Vec<HashMap<u64, u64>> {
    let n = arr.len();
    let mut res = vec![HashMap::new(); n + 1];

    for mask in 0..(1usize << n) {
        let mut cnt = 0usize;
        let mut sum_mod = 0u64;

        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                cnt += 1;
                sum_mod = (sum_mod + arr[i]) % m;
            }
        }

        *res[cnt].entry(sum_mod).or_insert(0) += 1;
    }

    res
}

fn main() {
    input! {
        n: usize,
        k: usize,
        m: u64,
        a: [u64; n],
    }

    let mid = n / 2;
    let left = &a[..mid];
    let right = &a[mid..];

    let left_info = enumerate_half(left, m);
    let right_info = enumerate_half(right, m);

    let mut ans = 0u64;

    for take_left in 0..=k {
        if take_left > left.len() {
            continue;
        }
        let take_right = k - take_left;
        if take_right > right.len() {
            continue;
        }

        for (&rem_left, &cnt_left) in &left_info[take_left] {
            let need = (m - rem_left) % m;
            if let Some(&cnt_right) = right_info[take_right].get(&need) {
                ans = (ans + cnt_left * cnt_right) % MOD;
            }
        }
    }

    println!("{}", ans);
}