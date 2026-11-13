use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = vec![0; 3];
    for i in 1..n {
        if a[i] > a[i - 1] {
            ans[0] += 1;
        } else if a[i] < a[i - 1] {
            ans[1] += 1;
        } else {
            ans[2] += 1;
        }
    }

    println!("{}", ans.iter().format(" "));
}
