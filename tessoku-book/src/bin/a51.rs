use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut stack = Vec::new();
    for _ in 0..q {
        input! {
            t: u64,
        }
        if t == 1 {
            input! {
                x: String,
            }
            stack.push(x);
        } else if t == 2 {
            println!("{}", stack.last().unwrap());
        } else {
            stack.pop();
        }
    }
}
