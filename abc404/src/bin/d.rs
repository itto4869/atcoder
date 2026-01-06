use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [u64; n],
    }
    let mut animals = vec![Vec::new(); n * 2];
    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }

        for ai in a {
            animals[ai].push(i);
            animals[n + ai].push(i + m);
        }
    }

    let mut ans = u64::MAX;
    let mut counts = vec![0; m];
    dfs(0, n, m, &c, &animals, 0, &mut ans, &mut counts);

    println!("{}", ans);
}

fn dfs(v: usize, n: usize, m: usize, c: &Vec<u64>, animals: &Vec<Vec<usize>>, curr_cost: u64, ans: &mut u64, counts: &mut Vec<usize>) {
    if curr_cost >= *ans {
        return;
    }

    if v == n {
        if counts.iter().all(|&x| x >= 2) {
            *ans = min(*ans, curr_cost);
        }
        return;
    }
    for visits in 0..=2 {
        for &animal_id in &animals[v] {
            counts[animal_id] += visits;
        }

        dfs(
            v + 1,
            n,
            m,
            c,
            animals,
            curr_cost + c[v] * visits as u64,
            ans,
            counts,
        );

        for &animal_id in &animals[v] {
            counts[animal_id] -= visits;
        }
    }
}