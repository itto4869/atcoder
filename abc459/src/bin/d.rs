use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        let mut v = vec![0usize; 26];
        input! {
            s: Bytes,
        }

        let n = s.len();
        for c in s {
            v[(c - b'a') as usize] += 1;
        }

        let mut v: Vec<(usize, char)> = v.into_iter().enumerate().map(|(i, c)| (c, (b'a' + i as u8) as char)).collect();

        let mut pq = BinaryHeap::new();
        for (cnt, c) in v {
            if cnt > 0 {
                pq.push((cnt, c));
            }
        }
        
        let mut ok = true;
        let mut ans = Vec::new();
        while !pq.is_empty() {
            let (cnt, c) = pq.pop().unwrap();
            if let Some(&prev) = ans.last() {
                if c == prev {
                    if !pq.is_empty() {
                        let (ncnt, nc) = pq.pop().unwrap();
                        ans.push(nc);
                        pq.push((cnt, c));
                        if ncnt - 1 > 0 {
                            pq.push((ncnt - 1, nc));
                        }
                    } else {
                        ok = false;
                        break;
                    }
                } else {
                    ans.push(c);
                    if cnt - 1 > 0 {
                        pq.push((cnt - 1, c));
                    }
                }
            } else {
                ans.push(c);
                if cnt - 1 > 0 {
                    pq.push((cnt - 1, c));
                }
            }
        }

        if ok {
            println!("Yes");
            println!("{}", ans.iter().format(""));
        } else {
            println!("No");
        }
    }
}
