use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            continue;
        }

        ans += a[i - 1];
    }

    println!("{}", ans);
}
