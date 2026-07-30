use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; n]; m],
    }
    let mut v = vec![vec![true; n]; n];
    for i in 0..m {
        for j in 0..(n - 1) {
            let (idx1, idx2) = (a[i][j], a[i][j + 1]);
            v[idx1][idx2] = false;
            v[idx2][idx1] = false;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if v[i][j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
