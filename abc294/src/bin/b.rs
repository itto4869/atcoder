use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let mut s = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                continue;
            }

            s[i][j] = ('A' as u8 + a[i][j] as u8 - 1) as char;
        }
    }

    for v in s {
        println!("{}", v.iter().format(""));
    }
}
