use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [u64; n],
            b: [u64; n],
        }
        let mut queue = VecDeque::new();
        for i in 0..n {
            let mut bi = b[i];
            queue.push_back((a[i], i));
            while bi > 0 {
                let (k, j): (u64, usize) = queue.pop_front().unwrap();
                if (i - j) > d {
                    continue;
                }

                if k > bi {
                    queue.push_front((k - bi, j));
                    break;
                } else {
                    bi -= k;
                }
            }
        }
        let mut res = 0;
        for (k, j) in queue {
            if (n - j) <= d {
                res += k;
            }
        }

        println!("{}", res);
    }
}
