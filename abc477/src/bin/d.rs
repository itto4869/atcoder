use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut prev_c = 'a';
    let mut idx = 0;
    let mut v = vec![None; n];
    let mut tile = vec![(false, 0); n];
    for i in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1
            }
            if tile[x].0 {
                tile[x] = (false, i);
            } else {
                if idx < tile[x].1 {
                    tile[x] = (true, i);
                } else {
                    v[x] = Some(prev_c);
                    tile[x] = (true, i);
                }
            }
        } else {
            input! {
                c: char
            }
            prev_c = c;
            idx = i;
        }
    }

    for i in 0..n {
        if !tile[i].0 && tile[i].1 <= idx {
            v[i] = Some(prev_c);
        }
    }

    let ans: Vec<char> = v.into_iter().map(|c| c.unwrap()).collect();
    println!("{}", ans.iter().format(""));
}
