use proconio::{fastout, input};

const WMAX: usize = 2005;
const HMAX: usize = 2005;

#[fastout]
fn main() {
    input! {
        n: usize,
        udlr: [(usize, usize, usize, usize); n],
    }
    let mut cnt = vec![vec![0; WMAX]; HMAX];
    for &(u, d, l, r) in &udlr {
        cnt[u][l] += 1;
        cnt[u][r + 1] -= 1;
        cnt[d + 1][l] -= 1;
        cnt[d + 1][r + 1] += 1;
    }

    for i in 0..HMAX {
        for j in 1..WMAX {
            cnt[i][j] += cnt[i][j - 1];
        }
    }

    for j in 0..WMAX {
        for i in 1..HMAX {
            cnt[i][j] += cnt[i - 1][j];
        }
    }

    let mut ones = vec![vec![0; WMAX]; HMAX];
    let mut empty_cnt = 0;

    for i in 1..2001 {
        for j in 1..2001 {
            if cnt[i][j] == 0 {
                empty_cnt += 1;
            } else if cnt[i][j] == 1 {
                ones[i][j] = 1;
            }
        }
    }

    let mut s = vec![vec![0; WMAX]; HMAX];

    for i in 1..2002 {
        for j in 1..2002 {
            s[i][j] = s[i][j - 1] + s[i - 1][j] - s[i - 1][j - 1] + ones[i][j];
        }
    }

    for &(u, d, l, r) in &udlr {
        let cnt_1_cloud = s[d][r] - s[u - 1][r] - s[d][l - 1] + s[u - 1][l - 1];
        println!("{}", empty_cnt + cnt_1_cloud);
    }
}
