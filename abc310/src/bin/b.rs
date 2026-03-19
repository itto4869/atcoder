use std::collections::HashSet;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        _: usize,
    }
    let mut p = Vec::new();
    let mut f = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            pi: u64,
            c: usize,
        }
        p.push(pi);
        for _ in 0..c {
            input! {
                fi: u64,
            }
            f[i].push(fi);
        }
    }

    let mut set = HashSet::new();
    for comb in (0..n).permutations(2) {
        let mut ok = true;
        let (i, j) = (comb[0], comb[1]);
        let pi = p[i];
        let pj = p[j];

        if pi < pj {
            ok = false;
        }

        for &fj in &f[j] {
            set.insert(fj);
        }

        for fi in &f[i] {
            if !set.contains(fi) {
                ok = false;
            }
        }

        if pi == pj {
            if f[i].len() >= f[j].len() {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }

        set.clear();
    }

    println!("No");
}
