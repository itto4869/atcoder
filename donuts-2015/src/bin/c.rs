use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    for _ in 0..n {
        input! {
            h: u64,
        }
        println!("{}", stack.len());
        while let Some(top) = stack.pop() {
            if top > h {
                stack.push(top);
                break;
            }
        }
        stack.push(h);
    }
}
