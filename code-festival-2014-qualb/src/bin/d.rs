use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }
    let mut stack = Vec::new();
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut cnt = 1;
        while let Some((top, k)) = stack.pop() {
            if top > h[i] {
                stack.push((top, k));
                break;
            } else {
                cnt += k;
                continue;
            }
        }
        stack.push((h[i], cnt));

        if let Some(idx) = stack.len().checked_sub(1) {
            ans[i] += stack[idx].1 - 1;
        } 
    }

    stack.clear();
    for i in (0..n).rev() {
        let mut cnt = 1;
        while let Some((top, k)) = stack.pop() {
            if top > h[i] {
                stack.push((top, k));
                break;
            } else {
                cnt += k;
                continue;
            }
        }
        stack.push((h[i], cnt));

        if let Some(idx) = stack.len().checked_sub(1) {
            ans[i] += stack[idx].1 - 1;
        } 
    }

    println!("{}", ans.iter().format("\n"));
}
