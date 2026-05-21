use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let ans = dfs(n, &mut map);
    println!("{}", ans);
}

fn dfs(x: usize, map: &mut HashMap<usize, usize>) -> usize {
    if x < 2 {
        return 0;
    } else {
        if let Some(&sum) = map.get(&x) {
            return sum;
        } else {
            let a = dfs(x / 2, map);
            let b = dfs((x + 1) / 2, map);
            map.insert(x, a + b + x);
            return a + b + x;
        }
    }
}