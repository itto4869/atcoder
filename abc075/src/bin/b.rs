use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut ans = String::new();
    let dist: [(isize, isize); 8] = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    for i in 0..h {
        for j in 0..w {
            let p = grid[i][j];
            if p == b'#' {
                ans.push('#');
            } else {
                let mut count = 0u32;
                for (dy, dx) in dist {
                    let nx = j as isize + dx;
                    let ny = i as isize + dy;
                    if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
                        continue;
                    }
                    if grid[ny as usize][nx as usize] == b'#' {
                        count += 1;
                    } else {
                        continue;
                    }
                }
                ans.push(char::from_digit(count, 10).unwrap());
            }
        }
        ans.push('\n');
    }
    print!("{}", ans);
}
