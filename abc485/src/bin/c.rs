use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n],
    }
    let mut forward = vec![0; n];
    forward[0] = 1usize << 60;
    for i in 1..n {
        if c[i] == c[i - 1] {
            forward[i] = forward[i - 1] + 1;
        } else {
            forward[i] = 1;
        }
    }

    let mut backward = vec![0; n];
    backward[n - 1] = 1usize << 60;
    for i in (0..(n - 1)).rev() {
        if c[i] == c[i + 1] {
            backward[i] = backward[i + 1] + 1;
        } else {
            backward[i] = 1;
        }
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        ans.push(forward[i].min(backward[i]));
    }

    println!("{}", ans.iter().format(" "));
}
