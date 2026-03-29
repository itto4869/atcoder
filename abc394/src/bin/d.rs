use cp_library::utils::yes_no;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut stack = Vec::new();
    for &c in &s {
        if c == b'(' || c == b'[' || c == b'<' {
            stack.push(c);
        } else if stack.len() > 0 {
            if c == b')' && *stack.last().unwrap() == b'(' {
                stack.pop();
            } else if c == b']' && *stack.last().unwrap() == b'[' {
                stack.pop();
            } else if c == b'>' && *stack.last().unwrap() == b'<' {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    yes_no(stack.is_empty());
}
