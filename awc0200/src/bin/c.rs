use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let mut r = 0;
    let mut c = 0;
    loop {
        let mut nr = r;
        let mut nc = c;
        if grid[r][c] == 'v' {
            nr += 1;
        } else {
            nc += 1;
        }

        if nr >= h || nc >= w {
            println!("{} {}", r + 1, c + 1);
            break;
        }

        r = nr;
        c = nc;
    }
}
