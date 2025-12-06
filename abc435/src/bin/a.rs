use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 0..=n {
        ans += i;
    }

    println!("{}", ans);
}
