use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
        m: u128,
    }

    if n * n < m {
        println!("-1");
        return;
    }

    let mut ans = u128::MAX;
    let mut a = 1;

    loop {
        let b = (m + a - 1) / a;

        if b <= n {
            ans = ans.min(a * b);
        }

        // ceil(sqrt(m)) まで調べれば十分
        if a * a >= m || a == n {
            break;
        }

        a += 1;
    }

    println!("{}", ans);
}