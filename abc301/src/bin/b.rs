use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    for _ in 0..n {
        input! {
            ai: usize,
        }

        if a.is_empty() {
            a.push(ai);
        } else if a[a.len() - 1].abs_diff(ai) == 1 {
            a.push(ai);
        } else if a[a.len() - 1] < ai {
            for x in (a[a.len() - 1] + 1)..=ai {
                a.push(x);
            }
        } else if a[a.len() - 1] > ai {
            for x in (ai..a[a.len() - 1]).rev() {
                a.push(x);
            }
        }
    }

    println!("{}", a.iter().format(" "));
}
