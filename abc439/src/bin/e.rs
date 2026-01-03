use std::cmp::Reverse;
use cp_library::lis;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(u64, u64); n],
    }
    ab.sort_unstable_by_key(|x| (x.0, Reverse(x.1)));

    let bs: Vec<u64> = ab.iter().map(|&(a, b)| b).collect();
    let ans = lis(&bs);


    println!("{}", ans);

}

fn solve_lis(nums: &[u64]) -> usize {
    let mut dp = Vec::new();

    for &x in nums {
        let idx = dp.binary_search(&x).unwrap_or_else(|i| i);

        if idx < dp.len() {
            dp[idx] = x;
        } else {
            dp.push(x);
        }
    }
    dp.len()
}