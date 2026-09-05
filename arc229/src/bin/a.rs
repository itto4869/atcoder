use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }
    let mut v = vec![0; 50];
    for i in 0..25 {
        v[i] = 1;
    }

    for _ in 0..(625 - x) {
        for i in 0..(v.len() - 1) {
            if v[i] > v[i + 1] {
                v[i] = 0;
                v[i + 1] = 1;
                break;
            }
        }
    }

    let mut ans = vec!['R'; 100];
    for i in 0..100 {
        if (i + 1) % 2 == 0 {
            continue;
        }

        if v[i / 2] == 0 {
            ans[i] = 'C';
        } else {
            ans[i] = 'A';
        }
    }

    println!("{}", ans.iter().format(""));
}