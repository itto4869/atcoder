use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let (a1, b1) = ab[0];
    
    let mut cnt_a = vec![0usize; n];
    let mut cnt_b = vec![0usize; n];
    let mut num_a = 0;
    let mut num_b = 0;
    for i in 1..m {
        let (a, b) = ab[i];
        if (a == a1) || (b == a1) {
        } else {
            num_a += 1;
            cnt_a[a - 1] += 1;
            cnt_a[b - 1] += 1;
        }

        if (a == b1) || (b == b1) {
        } else {
            num_b += 1;
            cnt_b[a - 1] += 1;
            cnt_b[b - 1] += 1;
        }
    }

    let mut set = HashSet::new();

    for i in 0..n {
        if (i + 1) == a1 {
            continue;
        }
        let ca = cnt_a[i];
        if ca == num_a {
            set.insert((a1.min(i + 1), a1.max(i + 1)));
        }
    }

    for i in 0..n {
        if (i + 1) == b1 {
            continue;
        }
        let cb = cnt_b[i];
        if cb == num_b {
            set.insert((b1.min(i + 1), b1.max(i + 1)));
        }
    }

    let mut ans = set.len();  
    println!("{}", ans);
}
