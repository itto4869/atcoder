use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for mask in 0..(1 << n) {
        let mut seq = String::with_capacity(n);
        let mut left = 0;
        let mut right = 0;
        for i in 0..n {
            if mask & (1 << (n - i - 1)) == 0 {
                seq.push('(');
                left += 1;
            } else {
                seq.push(')');
                right += 1;
            }
            if left < right {
                break;
            }
        }
        if left == right {
            println!("{}", seq);
        }
    }
}
