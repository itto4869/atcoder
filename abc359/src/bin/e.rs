use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut stack = Vec::new();
    let mut sum = 0;
    let mut ans = Vec::new();

    for _ in 0..n {
        let mut cnt = 1;
        input! {
            h: usize,
        }

        while let Some(&(v, c)) = stack.last() {
            if v <= h {
                stack.pop();
                sum -= v * c;
                cnt += c;
            } else {
                break;
            }
        }

        stack.push((h, cnt));
        sum += h * cnt;

        ans.push(sum + 1);
    }

    println!("{}", ans.iter().format(" "));
}
