use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut ans = 1;
    while n >= ans * ans {
        ans += 1;
    }
    println!("{}", (ans - 1) * (ans - 1));
}
