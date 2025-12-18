use std::{collections::HashSet, u64};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[u64; n]; n],
        m: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        set.insert((a, b));
        set.insert((b, a));
    }

    let mut ans = u64::MAX;
    for perm in (0..n).permutations(n) {
        let mut ok = true;
        let mut cnt = 0;
        for i in 0..(n - 1) {
            if set.contains(&(perm[i], perm[i + 1])) {
                ok = false;
                break;
            } else {
                cnt += a[perm[i]][i];
            }
        }

        cnt += a[perm[n - 1]][n - 1];

        if ok {
            ans = ans.min(cnt);
        } else {
            continue;
        }
    }

    if ans == u64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
