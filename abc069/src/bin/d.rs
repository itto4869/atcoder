use proconio::{fastout, input};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut a: [usize; n],
    }
    let mut grid = vec![vec![0; w]; h];
    let mut dest = 1;
    let mut point: (i32, i32) = (0, -1);
    for (i, ai) in a.iter().enumerate() {
        for _ in 0..*ai {
            point = (point.0, point.1 + dest);
            grid[point.0 as usize][point.1 as usize] = i + 1;
            if dest == 1 && point.1 == (w - 1) as i32 {
                point = (point.0 + 1, w as i32);
                dest = -1;
            } else if dest == -1 && point.1 == 0 {
                point = (point.0 + 1, -1);
                dest = 1;
            } else {
                continue;
            }
        }
    }

    for row in grid {
        println!("{}", row.iter().format(" "));
    }
}
