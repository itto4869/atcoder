use std::io::{self, Read};

fn can_achieve(p: f64, win_prefix: &[usize], k: usize) -> bool {
    let n = win_prefix.len() - 1;

    // 左端の直前として使用可能な最大の添字
    let mut left_limit = 0usize;

    // P[t] = win_prefix[t] - p * t
    // 使用可能な t の範囲における最小値
    let mut min_prefix = 0.0f64; // P[0] = 0

    for r in 1..=n {
        if win_prefix[r] < k {
            continue;
        }

        // 区間 (t, r] に K 勝以上含まれる条件は
        // win_prefix[t] <= win_prefix[r] - K
        let target = win_prefix[r] - k;

        // win_prefix は単調非減少なので、
        // 使用可能な t の範囲を右へ広げていく
        while left_limit + 1 <= r
            && win_prefix[left_limit + 1] <= target
        {
            left_limit += 1;

            let value =
                win_prefix[left_limit] as f64
                - p * left_limit as f64;

            min_prefix = min_prefix.min(value);
        }

        let current =
            win_prefix[r] as f64
            - p * r as f64;

        // P[r] - P[t] >= 0 となる t が存在する
        if current >= min_prefix {
            return true;
        }
    }

    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let s = iter.next().unwrap().as_bytes();

    // win_prefix[i]:
    // 最初の i 回のゲームに含まれる勝利数
    let mut win_prefix = vec![0usize; n + 1];

    for i in 0..n {
        win_prefix[i + 1] =
            win_prefix[i] + usize::from(s[i] == b'o');
    }

    let mut low = 0.0f64;
    let mut high = 1.0f64;

    // 誤差は最大でも 1 / 2^60 程度
    for _ in 0..60 {
        let mid = (low + high) / 2.0;

        if can_achieve(mid, &win_prefix, k) {
            low = mid;
        } else {
            high = mid;
        }
    }

    println!("{low:.15}");
}