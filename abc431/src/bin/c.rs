use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [u64; n],
        mut b: [u64; m],
    }
    h.sort_unstable();
    h.reverse();
    b.sort_unstable();
    b.reverse();

    let mut i = 0;
    let mut j = 0;
    let mut cnt = 0;
    while i < n && j < m {
        if h[i] <= b[j] {
            cnt += 1;
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }

    if cnt >= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
