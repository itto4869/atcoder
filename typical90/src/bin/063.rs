use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[Usize1; w]; h],
    }
    let mut ans = 0;
    for i in 0..h {
        let mut count = vec![0; w * h];
        for j in 0..w {
            let p = grid[i][j];
            count[p] += 1;
            ans = ans.max(count[p]);
        }
    }

    for n in 2..=h {
        for comb in (0..h).combinations(n) {
            let mut count = vec![0; w * h];
            for j in 0..w {
                let mut ok = true;
                for idxs in comb.windows(2) {
                    if grid[idxs[0]][j] != grid[idxs[1]][j] {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    let p = grid[comb[0]][j];
                    count[p] += n;
                    ans = ans.max(count[p]);
                }
            }
        }
    }

    println!("{}", ans);
}
