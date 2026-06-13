use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = vec![Vec::new(); n];
    for i in 1..=n {
        input! {
            k: usize,
        }
        for _ in 0..k {
            input! {
                a: Usize1
            }
            ans[a].push(i);
        }
    }

    for v in ans {
        if v.is_empty() {
            println!("{}", v.len());
        } else {
            println!("{} {}", v.len(), v.iter().format(" "));
        }
    }
}
