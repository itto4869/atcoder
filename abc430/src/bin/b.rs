use std::collections::HashSet;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Bytes; n],
    }
    let mut count = HashSet::new();
    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            let mut clip = vec![Vec::new(); m];
            for k in i..i+m {
                for l in j..j+m {
                    clip[k-i].push(grid[k][l]);
                }
            }
            count.insert(clip);
        }
    }
    println!("{}", count.len());
}
