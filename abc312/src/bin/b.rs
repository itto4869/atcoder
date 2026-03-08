use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Bytes; n],
    }
    for i in 0..=(n - 9) {
        for j in 0..=(m - 9) {
            let mut ok = true;

            for k in 0..3 {
                for l in 0..3 {
                    if grid[i + k][j + l] != b'#' {
                        ok = false;
                    }

                    if grid[i + 8 - k][j + 8 - l] != b'#' {
                        ok = false;
                    }
                }

                if grid[i + k][j + 3] != b'.' {
                    ok = false;
                }

                if grid[i + 8 - k][j + 5] != b'.' {
                    ok = false;
                }
            }

            for l in 0..4 {
                if grid[i + 3][j + l] != b'.' {
                    ok = false;
                }

                if grid[i + 5][j + 8 - l] != b'.' {
                    ok = false;
                }
            }

            if ok {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
