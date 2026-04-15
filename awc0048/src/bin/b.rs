use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u64; n],
        d: [u64; n],
    }
    let mut ans = 0;
    let mut idx = 0;
    while idx < n {
        if h[idx] > 0 && d[idx] > 0 {
            ans += 1;
        }

        if idx + 2 < n && !(h[idx + 2] > 0 && d[idx + 2] > 0) {
            idx += 2;
            continue;
        }

        if idx + 1 < n && !(h[idx + 1] > 0 && d[idx + 1] > 0) {
            idx += 1;
            continue;
        }

        if idx + 2 < n {
            idx += 2;
        } else {
            idx += 1;
        }
    }

    println!("{}", ans);
}
