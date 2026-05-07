use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let mut l_cnt = 0;
    let mut stack = Vec::with_capacity(n);
    for s in s {
        if s == '(' {
            l_cnt += 1;
            stack.push(s);
        } else if s == ')' {
            if l_cnt > 0 {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        l_cnt -= 1;
                        break;
                    }
                }
            } else {
                stack.push(s);
            }
        } else {
            stack.push(s);
        }
    }

    if !stack.is_empty() {
        println!("{}", stack.iter().format(""));
    }
}
