use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut curr = vec![0i64; m];
    let mut next = vec![0i64; m];
    for _ in 0..n {
        input! {
            a: Usize1,
            b: Usize1
        }
        curr[a] += 1;
        next[b] += 1;
    }

    for i in 0..m {
        println!("{}", next[i] - curr[i]);
    }
}
