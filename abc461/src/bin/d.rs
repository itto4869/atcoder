use proconio::{fastout, input, marker::Bytes};

#[inline]
fn rect_sum(
    sum: &Vec<Vec<usize>>,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) -> usize {
    // [r1, r2) × [c1, c2) の和
    sum[r2][c2] + sum[r1][c1] - sum[r2][c1] - sum[r1][c2]
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        grid: [Bytes; h],
    }

    // 2次元累積和
    let mut sum = vec![vec![0usize; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            let x = (grid[i][j] - b'0') as usize;

            sum[i + 1][j + 1] =
                sum[i][j + 1]
                + sum[i + 1][j]
                - sum[i][j]
                + x;
        }
    }

    let mut ans: u64 = 0;

    // 左上座標を固定
    for r1 in 0..h {
        for c1 in 0..w {
            // 下端を伸ばす
            for r2 in r1 + 1..=h {
                // c2 は右端の「1つ右」を表す
                // つまり長方形は [r1, r2) × [c1, c2)

                // first c2 such that sum >= k
                let mut low = c1 + 1;
                let mut high = w + 1;

                while low < high {
                    let mid = (low + high) / 2;

                    if rect_sum(&sum, r1, c1, r2, mid) >= k {
                        high = mid;
                    } else {
                        low = mid + 1;
                    }
                }

                let left = low;

                // first c2 such that sum > k
                let mut low = c1 + 1;
                let mut high = w + 1;

                while low < high {
                    let mid = (low + high) / 2;

                    if rect_sum(&sum, r1, c1, r2, mid) > k {
                        high = mid;
                    } else {
                        low = mid + 1;
                    }
                }

                let right = low;

                // sum == k となる c2 は [left, right)
                ans += (right - left) as u64;
            }
        }
    }

    println!("{}", ans);
}