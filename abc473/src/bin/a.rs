use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i in (n / 2)..n {
        ans += a[i];
    }

    println!("{}", ans);
}
