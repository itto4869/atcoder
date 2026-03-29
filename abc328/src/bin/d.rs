use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut stack = Vec::new();
    for &c in &s {
        stack.push(c);
        loop {
            if stack.len() < 3 {
            break;
            }
            let n = stack.len();
            if stack[n - 1] == 'C' && stack[n - 2] == 'B' && stack[n - 3] == 'A' {
                stack.pop();
                stack.pop();
                stack.pop();
            } else {
                break;
            }
        }
    }

    let ans: String = stack.iter().collect();
    if !ans.is_empty() {
        println!("{}", ans);
    }
}
