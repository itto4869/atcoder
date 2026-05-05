use std::collections::VecDeque;

use cp_library::{grid::neighbors4, yes_no};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],   
    }
    let mut seen = vec![vec![false; w]; h];
    let map = |curr, next: char| {
        match curr {
            's' => next == 'n',
            'n' => next == 'u',
            'u' => next == 'k',
            'k' => next == 'e',
            'e' => next == 's',
            _ => false,
        }
    };

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    seen[0][0] = true;
    while let Some((r, c)) = queue.pop_front() {
        let next = neighbors4(r, c, h, w);
        for (nr, nc) in next {
            if seen[nr][nc] {
                continue;
            }

            let curr_c = grid[r][c];
            let next_c = grid[nr][nc];

            if !map(curr_c, next_c) {
                continue;
            }

            seen[nr][nc] = true;
            queue.push_back((nr, nc));
        }
    }

    yes_no!(seen[h - 1][w - 1]);
}
