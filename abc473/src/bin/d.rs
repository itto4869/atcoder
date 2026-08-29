use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut v = Vec::with_capacity(n);
    let mut ans = Vec::new();
    dfs(n, k, 0, &mut v, &mut ans, n);

    ans.sort_unstable();
    for u in ans {
        println!("{} ", u.iter().format(" "));
        
    }
}

fn dfs(i: usize, k: usize, sum: usize, v: &mut Vec<usize>, ans: &mut Vec<Vec<usize>>, n: usize) {
    if v.len() == (n - 1) {
        v.push(k - sum);
        
        let mut nv = v.clone();
        nv.reverse();
        ans.push(nv);

        v.pop();

        return;
    }
    let max_ai = (k - sum) / i;
    for ai in (0..=max_ai).rev() {
        let d = i * ai;
        if sum + d > k {
            return;
        }

        v.push(ai);
        dfs(i - 1, k, sum + d, v, ans, n);
        v.pop();
    }
}