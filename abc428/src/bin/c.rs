use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut s = VecDeque::new();
    let mut left = 0;
    let mut right = 0;
    let mut d_right = 0;
    let mut ok = true;
    for _ in 0..q {
        input! {
            n: u64,
        }
        if n == 1 {
            input! {
                c: char,
            }
            s.push_back(c);
            if c == '(' {
                left += 1;
            } else {
                right += 1;
                if !ok {
                    d_right += 1;
                }
            }
        } else {
            let c = s.pop_back().unwrap();
            if c == '(' {
                left -= 1;
            } else {
                right -= 1;
                if !ok {
                    d_right -= 1;
                }
            }
        }

        if !ok && d_right == 0 {
            ok = true;
        } else if ok {
            if left < right {
                ok = false;
                d_right += 1;
            }
        } else {}

        if ok && left == right {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
