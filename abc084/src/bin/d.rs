use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let x: usize = 100000;
    let mut flags = vec![true; x + 1];
    flags[0] = false;
    flags[1] = false;
    let sqrt_x = (f64::sqrt(x as f64) + 0.1).ceil() as usize;
    for p in 2..=sqrt_x {
        if !flags[p] {
            continue;
        }
        for mult in ((p * p)..=x).step_by(p) {
            flags[mult] = false;
        }
    }

    let mut cnt = vec![0; x + 1];
    for i in (3..=x).step_by(2) {
        if flags[i] && flags[(i + 1) / 2] {
            cnt[i] = cnt[i - 2] + 1;
        } else {
            cnt[i] = cnt[i - 2];
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let res = cnt[r] - cnt[l.saturating_sub(2)];
        println!("{}", res);
    }
}
