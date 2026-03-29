use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    stack.push((0, 1));
    for _ in 0..n {
        input! {
            a: u64,
        }
        if a == stack.last().unwrap().0 {
            let cnt = stack.last().unwrap().1;
            stack.push((a, cnt + 1));
        } else {
            stack.push((a, 1));
        }

        if stack.last().unwrap().0 == stack.last().unwrap().1 {
            let cnt = stack.last().unwrap().1;
            for _ in 0..cnt {
                stack.pop();
            }
        }

        let ans = stack.len() - 1;
        println!("{}", ans);
    }
}
