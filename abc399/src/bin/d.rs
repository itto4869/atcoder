use std::collections::HashSet;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; 2 * n],
        }
        let mut position = vec![Vec::new(); n];
        for i in 0..(2 * n) {
            position[a[i]].push(i);
        }

        let mut set = HashSet::new();

        for i in 0..(2 * n - 1) {
            let l = a[i];
            let r = a[i + 1];
            if (position[l][0] + 1 == position[l][1]) || (position[r][0] + 1 == position[r][1]) {
                continue;
            }

            let mut v = [position[l][0], position[l][1], position[r][0], position[r][1]];
            v.sort_unstable();

            if (v[0] + 1 == v[1]) && (v[2] + 1 == v[3]) {
                set.insert((l.min(r), l.max(r)));
            }
        }

        println!("{}", set.len());
    }
}
