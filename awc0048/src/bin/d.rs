use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        grid: [Bytes; n],
    }
    let mut grid_sum = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if j == 0 {
                grid_sum[i][j] = (grid[i][j] - b'0') as i64;
            } else {
                grid_sum[i][j] += grid_sum[i][j - 1] + (grid[i][j] - b'0') as i64;
            }
        }
    }

    let mut ans = -1;
    for r in 1..=k {
        if k % r != 0 {
            continue;
        }

        let c = k / r;
        if r > n {
            continue;
        }

        if c > m {
            continue;
        }

        for i in 0..=(n - r) {
            for j in 0..=(m - c) {
                let mut cnt = 0;
                for k in 0..r {
                    if j == 0 {
                        cnt += grid_sum[i + k][j + c - 1];
                    } else {
                        cnt += grid_sum[i + k][j + c - 1] - grid_sum[i + k][j - 1];
                    }
                }
                ans = ans.max(cnt);
            }
        }
    }

    println!("{}", ans);
}
