use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let mut ans = vec!['x'; h];
    for i in 0..h {
        if grid[i].contains(&'T') {
            ans[i] = 'o';
        }
    }

    println!("{}", ans.iter().format(""));
}
