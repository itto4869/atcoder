use ac_library::Dsu;
use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut dsu = Dsu::new(h * w);
    let mut grid = vec![vec![0; w]; h];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                r: Usize1,
                c: Usize1,
            }

            grid[r][c] = 1;
            let neighbors = neighbors4(r, c, h, w);
            for (nr, nc) in neighbors {
                if grid[nr][nc] == 1 {
                    dsu.merge(w * r + c, w * nr + nc);
                }
            }
        } else {
            input! {
                ra: Usize1,
                ca: Usize1,
                rb: Usize1,
                cb: Usize1,
            }

            if grid[ra][ca] == 1 && grid[rb][cb] == 1 && dsu.same(w * ra + ca, w * rb + cb) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
