use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut left = 0;
    let mut ans = 0;

    for right in 0..n {
        *map.entry(a[right]).or_insert(0) += 1;

        while map.len() > k {
            if let Some(count) = map.get_mut(&a[left]) {
                *count -= 1;

                if *count == 0 {
                    map.remove(&a[left]);
                }
            }
            left += 1;
        }

        ans = ans.max(right - left + 1);
    }

    println!("{}", ans);
}
