use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: u64,
        s: Bytes,
    }
    let mut stack = Vec::with_capacity(n);
    for &c in &s {
        if c == b'U' {
            if let Some(t) = stack.pop() {
                if t == b'U' {
                    stack.push(t);
                    stack.push(c);
                }
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    for &c in &stack {
        if c == b'U' {
            x /= 2;
        } else if c == b'L' {
            x *= 2;
        } else {
            x = 2 * x + 1;
        }
    }

    println!("{}", x);
}
