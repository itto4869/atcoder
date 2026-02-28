use std::collections::{HashMap, HashSet};

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut map = HashMap::new();
    for &c in &s {
        *map.entry(c).or_insert(0) += 1;
    }

    let mut max_n = 0;
    for (_, &cnt) in &map {
        max_n = max_n.max(cnt);
    }

    let mut delete_c = HashSet::new();
    for (c, cnt) in map {
        if cnt == max_n {
            delete_c.insert(c);
        }
    }

    let mut ans = String::new();
    for &c in &s {
        if !delete_c.contains(&c) {
            ans.push(c);
        }
    }

    if !ans.is_empty() {
        println!("{}", ans);
    }
}
