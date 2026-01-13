use std::{collections::HashMap, u64};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
        d_grid: [[u64; c]; c],
        c_grid: [[Usize1; n]; n],
    }
    let mut group_1 = HashMap::new();
    let mut group_2 = HashMap::new();
    let mut group_3 = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            if (i + j) % 3 == 0 {
                *group_1.entry(c_grid[i][j]).or_insert(0) += 1;
            } else if (i + j) % 3 == 1 {
                *group_2.entry(c_grid[i][j]).or_insert(0) += 1;
            } else {
                *group_3.entry(c_grid[i][j]).or_insert(0) += 1;
            }
        }
    }

    let colors: Vec<usize> = (0..c).collect();
    let mut ans = u64::MAX;
    for color in colors.iter().permutations(3) {
        let (&c1, &c2, &c3) = (color[0], color[1], color[2]);
        let mut cnt = 0;
        for (&prev_c, &k) in &group_1 {
            cnt += d_grid[prev_c][c1] * k;
        }

        for (&prev_c, &k) in &group_2 {
            cnt += d_grid[prev_c][c2] * k;
        }

        for (&prev_c, &k) in &group_3 {
            cnt += d_grid[prev_c][c3] * k;
        }

        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
