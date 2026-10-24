use std::cmp::Ordering::{Equal, Greater, Less};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
        }
        let mut t = Vec::with_capacity(n);
        let mut b = Vec::with_capacity(n);
        for _ in 0..n {
            input! {
                x: i64,
                y: i64,
            }
            if y >= 0 {
                t.push((x, y));
            } else {
                b.push((x, y));
            }
        }

        t.sort_by(|a, b| {
            let d = a.0 * b.1 - a.1 * b.0;
            if d == 0 {
                Equal
            } else if d > 0 {
                Less
            } else {
                Greater
            }
        });

        b.sort_by(|a, b| {
            let d = a.0 * b.1 - a.1 * b.0;
            if d == 0 {
                Equal
            } else if d > 0 {
                Less
            } else {
                Greater
            }
        });

        let mut ans = 0;
        //println!("{:?}, {:?}", t, b);
        for i in 0..t.len() {
            let p = t[i];
            let mut r = i + 1;
            let mut l = t.len() - i;
            let idx = b.partition_point(|&x| {
                let d = p.0 * x.1 - p.1 * x.0;
                if d > 0 {
                    true
                } else {
                    false
                }
            });
            l += idx;
            r += b.len() - idx;
            //println!("{:?} {} {} {}", p, l, r, idx);
            ans = ans.max(l).max(r);
        }

        for i in 0..b.len() {
            let p = b[i];
            let mut l = i + 1;
            let mut r = b.len() - i;
            let idx = t.partition_point(|&x| {
                let d = p.0 * x.1 - p.1 * x.0;
                if d > 0 {
                    true
                } else {
                    false
                }
            });

            l += t.len() - idx;
            r += idx;
            //println!("{:?} {} {} {}", p, l, r, idx);
            ans = ans.max(l).max(r);
        }

        println!("{}", ans);
    }
}
