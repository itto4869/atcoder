use std::collections::VecDeque;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut row_cnt = vec![vec![0; 26]; h];
    let mut col_cnt = vec![vec![0; 26]; w];
    let mut row_rem = vec![w; h];
    let mut col_rem = vec![h; w];
    let mut row_types = vec![0; h];
    let mut col_types = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            let c = (grid[i][j] - b'a') as usize;

            if row_cnt[i][c] == 0 { row_types[i] += 1; }
            row_cnt[i][c] += 1;

            if col_cnt[j][c] == 0 { col_types[j] += 1; }
            col_cnt[j][c] += 1;
        }
    }

    let mut q = VecDeque::new();
    let mut row_queued = vec![false; h];
    let mut col_queued = vec![false; w];

    for i in 0..h {
        if row_rem[i] >= 2 && row_types[i] == 1 {
            q.push_back((true, i));
            row_queued[i] = true;
        }
    }
    for j in 0..w {
        if col_rem[j] >= 2 && col_types[j] == 1 {
            q.push_back((false, j));
            col_queued[j] = true;
        }
    }

    let mut deleted = vec![vec![false; w]; h];
    let mut ans = h * w;

    while let Some((is_row, idx)) = q.pop_front() {
        if is_row {
            let i = idx;
            for j in 0..w {
                if !deleted[i][j] {
                    deleted[i][j] = true;
                    ans -= 1;

                    let c = (grid[i][j] - b'a') as usize;
                    col_cnt[j][c] -= 1;
                    if col_cnt[j][c] == 0 {
                        col_types[j] -= 1;
                    }
                    col_rem[j] -= 1;

                    if !col_queued[j] && col_rem[j] >= 2 && col_types[j] == 1 {
                        q.push_back((false, j));
                        col_queued[j] = true;
                    }
                }
            }
        } else {
            let j = idx;
            for i in 0..h {
                if !deleted[i][j] {
                    deleted[i][j] = true;
                    ans -= 1;

                    let c = (grid[i][j] - b'a') as usize;
                    row_cnt[i][c] -= 1;
                    if row_cnt[i][c] == 0 {
                        row_types[i] -= 1;
                    }
                    row_rem[i] -= 1;

                    if !row_queued[i] && row_rem[i] >= 2 && row_types[i] == 1 {
                        q.push_back((true, i));
                        row_queued[i] = true;
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
