use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    }
    let mut m_cnt = vec![vec![0; 3]; n];
    let mut x_cnt = vec![vec![0; 3]; n];
    for i in 0..n {
        if i == 0 {
            if s[0] == 'M' {
                m_cnt[0][a[0]] = 1;
            }
        } else {
            m_cnt[i][0] = m_cnt[i - 1][0];
            m_cnt[i][1] = m_cnt[i - 1][1];
            m_cnt[i][2] = m_cnt[i - 1][2];
            if s[i] == 'M' {
                m_cnt[i][a[i]] += 1;
            }
        }
    }

    for i in (0..n).rev() {
        if i == (n - 1) {
            if s[i] == 'X' {
                x_cnt[n - 1][a[i]] = 1;
            }
        } else {
            x_cnt[i][0] = x_cnt[i + 1][0];
            x_cnt[i][1] = x_cnt[i + 1][1];
            x_cnt[i][2] = x_cnt[i + 1][2];
            if s[i] == 'X' {
                x_cnt[i][a[i]] += 1;
            }
        }
    }

    let mut ans = 0;
    for j in 1..(n - 1) {
        if s[j] == 'E' {
            for i in 0..3 {
                for k in 0..3 {
                    ans += m_cnt[j][i] * x_cnt[j][k] * mex(i, a[j], k)
                }
            }
        }
    }

    println!("{}", ans);
}

fn mex(ai: usize, aj: usize, ak: usize) -> usize {
    if !(0 == ai || 0 == aj || 0 == ak) {
        0
    } else if !(1 == ai || 1 == aj || 1 == ak) {
        1
    } else if !(2 == ai || 2 == aj || 2 == ak) {
        2
    } else {
        3
    }
}