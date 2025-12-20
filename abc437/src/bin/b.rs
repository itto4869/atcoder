use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        grid: [[u64; w]; h],
        b: [u64; n]
    }
    let mut ans = 0;
    for i in 0..h {
        let mut cnt = 0;
        for j in 0..w {
            let a = grid[i][j];
            for &bi in &b {
                if a == bi {
                    cnt += 1;
                }
            }
        }

        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
