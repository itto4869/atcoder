use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let dest = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'#' {
                let mut ok = false;
                for (dy, dx) in dest {
                    if let (Some(ny), Some(nx)) = (i.checked_add_signed(dy), j.checked_add_signed(dx)) {
                        if ny >= h || nx >= w {
                            continue;
                        }

                        if grid[ny][nx] == b'#' {
                            ok = true;
                            break;
                        }
                    }
                }

                if !ok {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
