use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [String; n],
    }
    let mut v = Vec::new();
    let perms: Vec<Vec<usize>> = std::iter::repeat(0..n)
        .take(k)
        .multi_cartesian_product()
        .collect();
    
    for p in perms {
        let mut tmp = String::new();
        for i in p {
            tmp.push_str(&s[i]);
        }
        v.push(tmp);
    }

    v.sort_unstable();
    let ans = &v[x];
    println!("{}", ans);
}