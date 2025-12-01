use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        grid: [[usize; w]; h],
        q: usize,
    }
    let mut map = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            map.insert(grid[i][j], (i, j));
        }
    }
    let mut cost = vec![0; h * w + 1];
    for i in (d + 1)..=(h * w) {
        let prev = map.get(&(i - d)).unwrap();
        let curr = map.get(&i).unwrap();
        cost[i] += cost[i - d] + prev.0.abs_diff(curr.0) + prev.1.abs_diff(curr.1);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let ans = cost[r] - cost[l];
        println!("{}", ans);
    }
}
