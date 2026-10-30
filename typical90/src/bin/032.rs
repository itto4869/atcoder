use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..m {
        input! {
            x: Usize1,
            y: Usize1,
        }
        set.insert((x, y));
        set.insert((y, x));
    }

    let mut ans = usize::MAX;
    for perm in (0..n).permutations(n) {
        let mut cnt = 0;
        let mut ok = true;
        for i in 0..(n - 1) {
            if set.contains(&(perm[i], perm[i + 1])) {
                ok = false;
                break;
            }

            cnt += a[perm[i]][i];
        }

        if ok {
            ans = ans.min(cnt + a[perm[n - 1]][n - 1]);
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
