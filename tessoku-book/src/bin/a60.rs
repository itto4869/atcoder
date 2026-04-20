use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            a: u64,
        }
        while let Some((top, idx)) = stack.pop() {
            if top < a {
                continue;
            } else {
                stack.push((top, idx));
                break;
            }
        }
        stack.push((a, i + 1));

        if let Some(idx) = stack.len().checked_sub(2) {
            ans.push(stack[idx].1 as isize);
        } else {
            ans.push(-1);
        }
    }

    println!("{}", ans.iter().format(" "));
}
