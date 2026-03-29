use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut stack = Vec::new();
    for _ in 0..100 {
        stack.push(0);
    }
    for _ in 0..q {
        input! {
            c: u64,
        }
        if c == 1 {
            input! {
                x: u64,
            }
            stack.push(x);
        } else {
            println!("{}", stack.pop().unwrap());
        }
    }
}
