use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: usize,
    }
    let mut ans = x;
    for _ in 0..y {
        ans *= 2;
    }

    println!("{}", ans);
}
