use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut dp0 = 0i64;
    let mut dp1 = -(1i64 << 60);

    for _ in 0..n {
        input! {
            x: usize,
            y: i64,
        }

        let mut ndp0 = dp0;
        let mut ndp1 = dp1;

        if x == 0 {
            ndp0 = ndp0.max(dp0 + y);

            ndp0 = ndp0.max(dp1 + y);
        } else {
            ndp1 = ndp1.max(dp0 + y);
        }

        dp0 = ndp0;
        dp1 = ndp1;
    }

    println!("{}", dp0.max(dp1));
}
