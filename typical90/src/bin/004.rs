use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut h_v = vec![0; h];
    for i in 0..h {
        let mut cnt = 0;
        for j in 0..w {
            cnt += a[i][j];
        }
        h_v[i] = cnt;
    }

    let mut w_v = vec![0; w];
    for j in 0..w {
        let mut cnt = 0;
        for i in 0..h {
            cnt += a[i][j];
        }
        w_v[j] = cnt;
    }

    let mut b = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            b[i][j] = h_v[i] + w_v[j] - a[i][j];
        }
    }

    for i in 0..b.len() {
        println!("{}", b[i].iter().format(" "));
    }
}
