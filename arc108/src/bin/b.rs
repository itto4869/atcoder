use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Bytes,
    }
    let mut stack = Vec::new();
    for &c in &s {
        stack.push(c);
        loop {
            let n = stack.len();
            if n < 3 {
                break;
            }

            if stack[n - 3] == b'f' && stack[n - 2] == b'o' && stack[n - 1] == b'x' {
                stack.pop();
                stack.pop();
                stack.pop();
            } else {
                break;
            }
        }
    }

    let ans = stack.len();
    println!("{}", ans);
}
