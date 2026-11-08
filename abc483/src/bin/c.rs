use std::{cmp::Reverse, collections::BTreeSet};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![0; n];
    let mut map_v = vec![Vec::new(); n];
    for &(a, b) in &ab {
        v[a] += 1;
        map_v[a].push(b);
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((Reverse(v[i]), i + 1));
    }

    for k in 0..n {
        for &b in &map_v[k] {
            set.remove(&(Reverse(v[k]), k + 1));
            set.insert((Reverse(v[k] - 1), k + 1));
            v[k] -= 1;

            set.remove(&(Reverse(v[b]), b + 1));
            set.insert((Reverse(v[b] + 1), b + 1));
            v[b] += 1;
        }

        let (_, idx) = set.first().unwrap();
        println!("{}", idx);

        for &b in &map_v[k] {
            set.remove(&(Reverse(v[k]), k + 1));
            set.insert((Reverse(v[k] + 1), k + 1));
            v[k] += 1;

            set.remove(&(Reverse(v[b]), b + 1));
            set.insert((Reverse(v[b] - 1), b + 1));
            v[b] -= 1;
        }
    }
}
