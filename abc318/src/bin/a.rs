use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize,
    }
    let mut curr = m;
    let mut ans = 0;
    while curr <= n {
        ans += 1;
        curr += p;
    }

    println!("{}", ans);
}
