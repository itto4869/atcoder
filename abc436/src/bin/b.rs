use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut grid = vec![vec![0; n]; n];
    let mut r = 0;
    let mut c = (n - 1) / 2;
    let mut k = 1;
    grid[r][c] = k;
    for _ in 0..(n * n - 1) {
        let mut nr = mod_minus(r as isize - 1, n as isize);
        let mut nc = (c + 1) % n;
        let nk = k + 1;
        if grid[nr][nc] == 0 {
            grid[nr][nc] = nk;
        } else {
            nr = (r + 1) % n;
            nc = c;
            grid[nr][nc] = nk;
        }

        r = nr;
        c = nc;
        k = nk;
    }

    for i in 0..n {
        println!("{}", grid[i].iter().format(" "));
    }
}

fn mod_minus(val: isize, m: isize) -> usize {
    let mut res = val % m;
    if res < 0 {
        res += m;
    }
    res as usize
}