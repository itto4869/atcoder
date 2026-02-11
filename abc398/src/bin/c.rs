use std::collections::{HashMap, HashSet};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for i in 0..n {
        input! {
            x: u64,
        }
        if set.contains(&x) {
            map.remove(&x);
        } else {
            map.insert(x, i);
            set.insert(x);
        }
    }

    let ans = map.into_iter().max_by(|x, y| x.0.cmp(&y.0));
    if let Some(ans) = ans {
        println!("{}", ans.1 + 1);
    } else {
        println!("-1");
    }
}
