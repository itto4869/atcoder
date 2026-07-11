use std::collections::{BinaryHeap, HashMap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            x: usize,
        }
        let mut pq = BinaryHeap::new();
        pq.push((x + 1, 1usize));

        for _ in 0..n {
            input! {
                a: usize,
            }

            let mut cnt = 0;
            while let Some((l, c)) = pq.pop() {
                if l <= a {
                    pq.push((l, c));
                    break;
                } else {
                    let q = l / a;
                    let r = l % a;

                    cnt += c * q;

                    if r > 0 {
                        pq.push((r, c));
                    }
                }
            }

            pq.push((a, cnt));
        }

        let mut ans = 0;
        for (l, c) in pq {
            ans += c;
        }

        println!("{}", ans - 1);
    }
}
