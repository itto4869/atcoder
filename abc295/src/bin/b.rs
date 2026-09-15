use std::collections::VecDeque;

use cp_library::grid::neighbors4;
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        grid: [Chars; r],
    }
    let mut ans = vec![vec![-1; c]; r];
    for i in 0..r {
        for j in 0..c {
            if grid[i][j] == '.' {
                ans[i][j] = 0;
                continue;
            } else if grid[i][j] == '#' {
                continue;
            }

            let n = grid[i][j].to_digit(10).unwrap() as i64;
            let mut queue = VecDeque::new();
            queue.push_back((n, (i, j)));
            ans[i][j] = ans[i][j].max(n);
            while let Some((n, (i, j))) = queue.pop_front() {
                if n == 0 {
                    continue;
                }

                let next = neighbors4(i, j, r, c);
                for (ni, nj) in next {
                    if ans[ni][nj] >= (n - 1) {
                        continue;
                    }

                    ans[ni][nj] = n - 1;
                    queue.push_back((n - 1, (ni, nj)));
                }
            }
        }
    }

    let mut ans_v: Vec<Vec<char>> = vec![Vec::new(); r];
    for i in 0..r {
        for j in 0..c {
            if ans[i][j] == -1 {
                ans_v[i].push('#');
            } else {
                ans_v[i].push('.');
            }
        }
    }

    for row in ans_v {
        println!("{}", row.iter().format(""));
    }
}
