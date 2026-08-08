use std::mem::swap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [usize; n],
    }
    let mut pp = vec![0; n];
    for i in 0..n {
        let pi = p[i];
        pp[pi - 1] = i + 1;
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
                y: Usize1,
            }

            let l = p[x] - 1;
            let r = p[y] - 1;

            let px = p[x];
            let py = p[y];
            p[x] = py;
            p[y] = px;

            let ppl = pp[l];
            let ppr = pp[r];
            pp[l] = ppr;
            pp[r] = ppl;
        } else {
            let new_p = pp;
            let new_pp = p;
            p = new_p;
            pp = new_pp;
        }
    }

    println!("{}", p.iter().format(" "));
}
