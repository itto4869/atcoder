use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Bytes; n],
        t: [Bytes; n],
    }
    let mut ans = 1 << 60;
    let mut cnt = 0u64;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                cnt += 1;
            }
        }
    }

    ans = ans.min(cnt);
    cnt = 3;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[n - j - 1][i] {
                cnt += 1;
            }
        }
    }

    ans = ans.min(cnt);
    cnt = 2;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[n - i - 1][n - j - 1] {
                cnt += 1;
            }
        }
    }

    ans = ans.min(cnt);
    cnt = 1;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[j][n - i - 1] {
                cnt += 1;
            }
        }
    }

    ans = ans.min(cnt);
    println!("{}", ans);
}
