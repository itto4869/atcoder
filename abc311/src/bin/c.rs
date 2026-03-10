use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut next = a[0];
    while !seen[next] {
        seen[next] = true;
        next = a[next];
    }

    let mut seen = vec![false; n];
    let mut ans = Vec::new();
    while !seen[next] {
        seen[next] = true;
        ans.push(next + 1);
        next = a[next];
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().format(" "));
}
