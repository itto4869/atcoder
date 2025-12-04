use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64
    }
    let mut ans = 0;
    if n == 1 && m == 1 {
        ans = 1;
    } else if n == 1 && m == 2 {
        ans = 0;
    } else if n == 1 || m == 1 {
        ans = n.max(m) - 2;
    } else if n == 2 || m == 2 {
        ans = 0;
    } else {
        ans = (n - 2) * (m - 2);
    }

    println!("{}", ans);
}
