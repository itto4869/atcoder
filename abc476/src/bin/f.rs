use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut imos = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            let k = (a[i] * b[j]) % m;
            let mut r = (n - i - 1).max(i).max(n - j - 1).max(j);
            imos[i][j] += k * r;
            imos[n][n] += k * r;
            imos[n][j] -= k * r;
            imos[i][n] -= k * r;

            let mut ii = n - i - 1;
            let mut jj = n - j - 1;
            while (ii, jj) != (0, 0) {
                if ii > jj {
                    ii -= 1;
                } else if ii < jj {
                    jj -= 1;
                } else {
                    ii -= 1;
                    jj -= 1;
                }

                r -= 1;
                imos[ii + i + 1][jj + j + 1] += k * r;
                imos[ii + i + 1][j] -= k * r;
                imos[i][jj + j + 1] -= k * r;
            }
        }
    }
}
