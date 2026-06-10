use std::process::exit;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    if k <= 9 {
        println!("{}", k);
    } else {
        let mut v = vec![Vec::new(); 9];
        for x in 1..=9 {
            v[x - 1].push(x);
        }
        let mut n = 9;
        dfs(&mut n, &mut v, k);
    }
}

fn dfs(n: &mut usize, v: &mut Vec<Vec<usize>>, k: usize) {
    let mut nv = Vec::new();
    for x in v {
        for y in x[x.len() - 1].saturating_sub(1)..=(x[x.len() - 1] + 1).min(9) {
            let mut cx = x.clone();
            cx.push(y);
            nv.push(cx);

            *n = *n + 1;
            if *n == k {
                println!("{}", nv[nv.len() - 1].iter().format(""));
                exit(0);
            }
        }
    }

    dfs(n, &mut nv, k);
}