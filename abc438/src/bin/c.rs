use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    stack.push((0, 0));
    for _ in 0..n {
        input! {
            a: u64,
        }
        if a == stack.last().unwrap().0 {
            let (m, cnt) = stack.pop().unwrap();
            stack.push((m, cnt + 1));
        } else {
            stack.push((a, 1));
        }

        if stack.last().unwrap().1 == 4 {
            stack.pop();
        }
    }

    let ans: u64 = stack.iter().map(|(_, b)| b).sum();
    println!("{}", ans);
}
