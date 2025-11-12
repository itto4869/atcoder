use proconio::{fastout, input, marker::Bytes};

fn shift(dp: &Vec<bool>, len: usize) -> Vec<bool> {
    if len == 0 {
        return dp.clone();
    }
    let n = dp.len();
    let mut next = vec![false; n];
    for i in 0..n {
        if dp[i] {
            if i + len < n {
                next[i + len] = true;
            }
            if i >= len {
                next[i - len] = true;
            }
        }
    }
    next
}

#[fastout]
fn main() {
    input! {
        s: Bytes,
        x: i64,
        y: i64,
    }

    let total_f = s.iter().filter(|&&c| c == b'F').count();
    
    let mut runs: Vec<usize> = Vec::new();
    let mut cnt = 0;
    for &ch in &s {
        if ch == b'F' {
            cnt += 1;
        } else {
            runs.push(cnt);
            cnt = 0;
        }
    }
    runs.push(cnt);

    let mut x_runs: Vec<usize> = Vec::new();
    let mut y_runs: Vec<usize> = Vec::new();
    for (i, &len) in runs.iter().enumerate() {
        if i % 2 == 0 {
            x_runs.push(len);
        } else {
            y_runs.push(len);
        }
    }

    let size = 2 * total_f + 1;
    let offset = total_f;

    let a0 = *x_runs.get(0).unwrap_or(&0);
    let mut dpx = vec![false; size];
    dpx[offset + a0] = true;
    for &len in x_runs.iter().skip(1) {
        if len > 0 {
            dpx = shift(&dpx, len);
        }
    }

    let mut dpy = vec![false; size];
    dpy[offset] = true;
    for &len in &y_runs {
        if len > 0 {
            dpy = shift(&dpy, len);
        }
    }

    let xi = (offset as i64 + x) as isize;
    let yi = (offset as i64 + y) as isize;
    let ok = xi >= 0 && xi < size as isize
        && yi >= 0 && yi < size as isize
        && dpx[xi as usize]
        && dpy[yi as usize];

    println!("{}", if ok { "Yes" } else { "No" });
}
