use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut res = Vec::with_capacity(n);
    dfs(1, 1, n, m, &mut res);
}

fn dfs(i: usize, k: usize, n: usize, m: usize, res: &mut Vec<usize>) {
    if i == n {
        for l in k..=m {
            res.push(l);
            println!("{}", res.iter().format(" "));
            res.pop();
        }
        return;
    }
    for l in k..=(m - (n - i)) {
        res.push(l);
        dfs(i + 1, l + 1, n, m, res);
        res.pop();
    }
}