use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }
    let mut ans = Vec::new();
    let mut v = Vec::new();
    dfs(0, 0, n, k, &r, &mut v, &mut ans);
    if !ans.is_empty() {
        println!("{}", ans.iter().format("\n"));
    }
}

fn dfs(idx: usize, sum: usize, n: usize, k: usize, r: &Vec<usize>, v: &mut Vec<usize>, ans: &mut Vec<String>) {
    if idx == (n - 1) {
        for x in 1..=r[idx] {
            if (sum + x) % k == 0 {
                v.push(x);
                ans.push(v.clone().iter().format(" ").to_string());
                v.pop();
            }
        }
        return;
    }

    for x in 1..=r[idx] {
        v.push(x);
        dfs(idx + 1, sum + x, n, k, r, v, ans);
        v.pop();
    }
}