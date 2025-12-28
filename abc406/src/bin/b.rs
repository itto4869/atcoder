use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u32,
    }
    let mut ans = 1;
    let max_ans = 10u128.pow(k);
    for _ in 0..n {
        input! {
            a: u128,
        }
        ans *= a;
        if ans >= max_ans {
            ans = 1;
        }
    }

    println!("{}", ans);
}
