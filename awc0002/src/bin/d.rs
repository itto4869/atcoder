use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut c: [u64; n],
        mut r: [u64; m],
    }
    c.sort_unstable();
    r.sort_unstable();
    let mut i = 0;
    let mut j = 0;

    let mut ans = 0;
    while i < n && j < m {
        if c[i] <= r[j] {
            ans += 1;
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    println!("{}", ans);
}
