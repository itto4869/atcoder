use cp_library::grid::neighbors4;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == '#' {
                continue;
            }
            let mut cnt = 0;
            let d = neighbors4(r, c, h, w);
            for (nr, nc) in d {
                if grid[nr][nc] == '#' {
                    cnt += 1;
                }
            }

            if cnt >= 2 {
                println!("{} {}", r + 1, c + 1);
                return;
            }
        }
    }
}
