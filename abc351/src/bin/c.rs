use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    for _ in 0..n {
        input! {
            a: u64,
        }
        stack.push(a);
        loop {
            if stack.len() <= 1 {
                break;
            } else if stack[stack.len() - 1] != stack[stack.len() - 2] {
                break;
            } else {
                let last = stack.pop().unwrap();
                stack.pop().unwrap();
                stack.push(last + 1);
            }
        }
    }

    println!("{}", stack.len());
}
