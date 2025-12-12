use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut boxes = vec![0; n];
    let mut ans = Vec::with_capacity(n);
    for _ in 0..q {
        input! {
            x: usize,
        }
        if x > 0 {
            boxes[x - 1] += 1;
            ans.push(x);
        } else {
            let mut key = 0;
            let mut min_value = 1000;
            for (k, &v) in boxes.iter().enumerate() {
                if v < min_value {
                    key = k;
                    min_value = v;
                }
            }
            boxes[key] += 1;
            ans.push(key + 1);
        }
    }

    println!("{}", ans.iter().format(" "));
}
