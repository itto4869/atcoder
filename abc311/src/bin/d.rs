use std::collections::{HashSet, VecDeque};

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Bytes; n],
    }
    let dest = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((1, 1));

    let mut seen = vec![vec![false; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let mut set = HashSet::new();
    while let Some((r, c)) = queue.pop_front() {
        seen[r][c] = true;
        visited[r][c] = true;
        for (dr, dc) in dest {
            let mut nr = r.wrapping_add_signed(dr);
            let mut nc = c.wrapping_add_signed(dc);
            if grid[nr][nc] == b'#' {
                continue;
            }

            if set.contains(&(r, c, nr, nc)) {
                continue;
            }

            set.insert((r, c, nr, nc));
            set.insert((nr, nc, r, c));
            
            while grid[nr][nc] == b'.' {
                seen[nr][nc] = true;
                nr = nr.wrapping_add_signed(dr);
                nc = nc.wrapping_add_signed(dc);
            }

            nr = nr.wrapping_add_signed(-dr);
            nc = nc.wrapping_add_signed(-dc);

            let pr = nr.wrapping_add_signed(-dr);
            let pc = nc.wrapping_add_signed(-dc);

            set.insert((pr, pc, nr, nc));
            set.insert((nr, nc, pr, nc));
            if !visited[nr][nc] {
                queue.push_back((nr, nc));
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if seen[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}