use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut stack = Vec::new();
    for _ in 0..q {
        input! {
            s: String,
        }
        if s == "READ" {
            println!("{}", stack.pop().unwrap());
        } else {
            stack.push(s);
        }
    }
}
