use std::collections::HashSet;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut point = HashSet::new();
    for _ in 0..n {
        input! {
            x: u64,
        }
        point.insert(x);
    }
    let mut point: Vec<u64> = point.into_iter().collect();
    point.sort();
    let mut dist = Vec::with_capacity(n);
    for i in 0..(point.len() - 1) {
        dist.push(point[i + 1] - point[i]);
    }
    dist.sort();

    let mut ans = 0;
    for i in 0..(point.len().saturating_sub(m)) {
        ans += dist[i];
    }
    println!("{}", ans);
}
