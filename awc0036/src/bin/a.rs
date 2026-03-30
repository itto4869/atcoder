use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut book = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            m: usize,
        }
        for _ in 0..m {
            input! {
                s: u64,
            }
            book[i].push(s);
        }
    }

    input! {
        q: usize,
    }
    let mut ans = 0;
    for _ in 0..q {
        input! {
            v: Usize1,
            d: Usize1,
        }
        if book[v][d] > 0 {
            book[v][d] -= 1;
        } else {
            ans += 1;
        }
    }

    for i in 0..n {
        println!("{}", book[i].iter().format(" "));
    }

    println!("{}", ans);
}
