use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        c: [usize; n],
        d: [usize; n - 1],
    }
    let mut ans = 0;
    let mut pq = BinaryHeap::new();
    let mut v = vec![0; n];
    let mut max_i = 0;
    for i in 0..(n - 1) {
        pq.push((Reverse(c[i]), Reverse(i)));
        let mut di = d[i];
        loop {
            //println!("{}", ans);
            if pq.is_empty() {
                println!("-1");
                return;
            }
            let (Reverse(ci), Reverse(i)) = pq.pop().unwrap();

            if i > max_i {
                continue;
            }

            if (t - v[i]) > di {
                v[i] += di;
                ans += ci * di;
                pq.push((Reverse(ci), Reverse(i)));
                break;
            } else if (t - v[i]) == di {
                v[i] += di;
                ans += ci * di;
                max_i = max_i.max(i);
                break;
            } else {
                v[i] = t;
                ans += ci * (t - v[i]);
                di -= t - v[i];

                max_i = max_i.max(i);
            }
        }
    }

    println!("{}", ans);
}
