use std::{collections::HashSet, iter::FromIterator};

use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..m {
        input! {
            x: Usize1,
            y: Usize1,
        }
        dsu.merge(x, y);
    }

    let groups = dsu.groups();
    let mut ans = 0;
    for group in groups {
        let group_set: HashSet<usize> = HashSet::from_iter(group.iter().cloned());

        for idx in group {
            if group_set.contains(&(p[idx] - 1)) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
