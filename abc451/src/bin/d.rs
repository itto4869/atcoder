use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut p = vec![Vec::new(); 10];
    let mut k = 0;
    while (2u64.pow(k)).ilog10() < 9 {
        let m = 2u64.pow(k);
        p[m.ilog10() as usize + 1].push(m);
        k += 1;
    }
    let mut sets = vec![HashSet::new(); 10];
    sets[0].insert(0);
    for k in 1..10 {
        let (prev, rest) = sets.split_at_mut(k);
        let cur = &mut rest[0]; // sets[k]

        for i in 1..=k {
            for &x in &prev[k - i] {
                for &pi in &p[i] {
                    cur.insert(x * 10u64.pow(i as u32) + pi);
                }
            }
        }
    }

    let mut cnt = 0;
    for k in 1..10 {
        if cnt + sets[k].len() >= n {
            let mut v: Vec<&u64> = sets[k].iter().collect();
            v.sort_unstable();
            let ans = v[n - cnt - 1];
            println!("{}", ans);
            break;
        } else {
            cnt += sets[k].len();
        }
    }
}
