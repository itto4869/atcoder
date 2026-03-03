use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut num = vec![0; n + 1];
    let mut vote = vec![Vec::new(); 37];
    for i in 0..n {
        input! {
            c: usize,
            a: [usize; c],
        }
        num[i + 1] = c;
        for &ai in &a {
            vote[ai].push(i + 1);
        }
    }

    input! {
        x: usize,
    }

    let sub = &vote[x];
    let mut v = Vec::new();
    for &idx in sub {
        v.push((num[idx], idx));
    }

    v.sort_unstable();

    if v.is_empty() {
        println!("0\n");
    } else {
        let max_num = v[0].0;
        let mut k = 0;
        let mut res = Vec::new();
        for (num, idx) in v {
            if num == max_num {
                k += 1;
                res.push(idx);
            }
        }

        println!("{}", k);
        println!("{}", res.iter().format(" "));
    }
}
