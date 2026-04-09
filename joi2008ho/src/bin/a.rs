use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c1: u64,
    }
    let mut stack = Vec::new();
    stack.push((c1, 1));
    for i in 2..=n {
        input! {
            c: u64,
        }
        let (prev_color, prev_cnt) = stack.pop().unwrap();
        if i % 2 == 0 {
            if c == prev_color {
                stack.push((prev_color, prev_cnt + 1));
            } else {
                if stack.is_empty() {
                    stack.push((c, prev_cnt + 1));
                } else {
                    let (pprev_color, pprev_cnt) = stack.pop().unwrap();
                    stack.push((pprev_color, pprev_cnt + prev_cnt + 1));
                }
            }
        } else {
            if c == prev_color {
                stack.push((prev_color, prev_cnt + 1));
            } else {
                stack.push((prev_color, prev_cnt));
                stack.push((c, 1));
            }
        }
    }

    let ans = stack.iter().filter(|(a, _)| *a == 0 ).map(|(_, b)| b ).sum::<u64>();
    println!("{}", ans);
}
