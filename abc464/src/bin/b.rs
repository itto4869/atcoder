use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let mut lx = 0;
    let mut ly = 0;
    let mut rx = 0;
    let mut ry = 0;

    let mut ok = true;
    for i in 0..h {
        ly = i;
        for j in 0..w {
            if grid[i][j] == '#' {
                ok = false;
            }
        }

        if !ok {
            break;
        }
    }

    let mut ok = true;
    for i in (ly..h).rev() {
        ry = i;
        for j in 0..w {
            if grid[i][j] == '#' {
                ok = false;
            }
        }

        if !ok {
            break;
        }
    }

    let mut ok = true;
    for j in 0..w {
        lx = j;
        for i in ly..=ry {
            if grid[i][j] == '#' {
                ok = false;
            }
        }

        if !ok {
            break;
        }
    }

    let mut ok = true;
    for j in (lx..w).rev() {
        rx = j;
        for i in ly..=ry {
            if grid[i][j] == '#' {
                ok = false;
            }
        }

        if !ok {
            break;
        }
    }

    for i in ly..=ry {
        for j in lx..=rx {
            print!("{}", grid[i][j]);
        }
        println!("");
    }
}
