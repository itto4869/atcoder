use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        k: u64,
    }
    let mut ans = 0;
    for x in 1..=n {
        let d_sum = digit_sum(x);
        if d_sum == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}

fn digit_sum(mut x: u64) -> u64 {
    let mut cnt = 0;
    while x > 0 {
        cnt += x % 10;
        x /= 10;
    }
    cnt
}