use proconio::{fastout, input, marker::Usize1};
use ac_library::dsu::Dsu;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut dsu = Dsu::new(h * w);
    let mut grid = vec![vec!["w"; w]; h];
    let d: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for _ in 0..q {
        input! {
            t: u64,
        }
        if t == 1 {
            input! {
                r: Usize1,
                c: Usize1,
            }
            grid[r][c] = "r";
            for &(dy, dx) in &d {
                if r as isize + dy < 0 || r as isize + dy >= h as isize || c as isize + dx < 0 || c as isize + dx >= w as isize {
                    continue;
                } else {
                    if grid[(r as isize + dy) as usize][(c as isize + dx) as usize] == "r" {
                        dsu.merge(w * r + c, w * (r as isize + dy) as usize + (c as isize + dx) as usize);
                    } else {
                        dsu.merge(w * r + c, w * r + c);
                    }
                }
            }
        } else {
            input! {
                ra: Usize1,
                ca: Usize1,
                rb: Usize1,
                cb: Usize1,
            }
            
            if grid[ra][ca] == "r" && grid[rb][cb] == "r" && dsu.same(w * ra + ca, w * rb + cb) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
