use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut v = vec![vec![n; 3]; n + 1];
    for i in (0..n).rev() {
        for j in 0..3 {
            v[i][j] = v[i + 1][j];
        }

        if s[i] == 'A' {
            v[i][0] = i;
        } else if s[i] == 'B' {
            v[i][1] = i;
        } else {
            v[i][2] = i;
        }
    }

    let mut end = n;
    for i in (0..n).rev() {
        let mut ok = true;
        for j in 0..3 {
            if v[i][j] == n {
                ok = false;
            }
        }

        if ok {
            end = i;
            break;
        }
    }

    if end == n {
        println!("0");
        return;
    }

    let mut start = 0;
    let mut cnt = vec![0; 3];
    for i in 0..n {
        if s[i] == 'A' {
            cnt[0] += 1;
        } else if s[i] == 'B' {
            cnt[1] += 1;
        } else {
            cnt[2] += 1;
        }

        let mut ok = true;
        for j in 0..3 {
            if cnt[j] == 0 {
                ok = false;
            }
        }

        if ok {
            start = i;
            break;
        }
    }

    let mut ans = 0;
    for i in (start + 1)..end {
        let idx = v[i][0].max(v[i][1]).max(v[i][2]);
        ans += end.saturating_sub(idx);
    }

    println!("{}", ans);
}
