use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = Vec::new();
    let mut ans: Vec<Vec<usize>> = Vec::new();
    dfs(&mut v, n, m, &mut ans);

    println!("{}", ans.len());
    for u in ans {
        println!("{}", u.iter().format(" "));
    }
}

fn dfs(v: &mut Vec<usize>, n: usize, m: usize, ans: &mut Vec<Vec<usize>>) {
    if v.len() == n {
        let vv = v.clone();
        ans.push(vv);
    } else if v.len() == 0 {
        for a in 1..=(m.saturating_sub(10 * (n - 1))) {
            v.push(a);
            dfs(v, n, m, ans);
            v.pop();
        }
    } else {
        for a in (v[v.len() - 1] + 10)..=(m.saturating_sub(10 * (n - (v.len() + 1)))) {
            v.push(a);
            dfs(v, n, m, ans);
            v.pop();
        }
    }
}