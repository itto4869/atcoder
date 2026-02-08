use std::collections::BTreeSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    }
    let mut ans = 0;
    let mut r = 0;
    let mut set = BTreeSet::new();
    set.insert(-1e9 as i64);
    set.insert(2e9 as i64);
    for l in 0..n {
        while r < n {
            if let Some(&next_val) = set.range(a[r]..).next() {
                if next_val - a[r] < d {
                    break;
                }
            }

            if let Some(&prev_val) = set.range(..a[r]).next_back() {
                if a[r] - prev_val < d {
                    break;
                }
            }

            set.insert(a[r]);
            r += 1;
        }
        ans += r - l;
        set.remove(&a[l]);
    }

    println!("{}", ans);
}
