use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let snuke = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            if i < (h - 4) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i + k][j] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i + k + 1, j + 1);
                    }
                }
            }

            if i >= 4 {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i - k][j] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i - k + 1, j + 1);
                    }
                }
            }

            if j < (w - 4) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i][j + k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i + 1, j + k + 1);
                    }
                }
            }

            if j >= 4 {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i][j - k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i + 1, j - k + 1);
                    }
                }
            }

            if (i < (h - 4)) && (j < (w - 4)) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i + k][j + k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i + k + 1, j + k + 1);
                    }
                }
            }

            if (i >= 4) && (j >= 4) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i - k][j - k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i - k + 1, j - k + 1);
                    }
                }
            }

            if (i < (h - 4)) && (j >= 4) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i + k][j - k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i + k + 1, j - k + 1);
                    }
                }
            }

            if (i >= 4) && (j < (w - 4)) {
                let mut ok = true;
                for k in 0..5 {
                    if grid[i - k][j + k] != snuke[k] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    for k in 0..5 {
                        println!("{} {}", i - k + 1, j + k + 1);
                    }
                }
            }
        }
    }
}
