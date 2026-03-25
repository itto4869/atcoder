use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }
    let mut cnt = 0u64;
    let mut p = 2;
    while p * p <= n {
        while n % p == 0 {
            n /= p;
            cnt += 1;
        }

        p += 1;
    }

    if n != 1 {
        cnt += 1;
    }

    let mut ans = 0;
    while cnt > 1 {
        cnt = (cnt + 1) / 2;
        ans += 1;
    }

    println!("{}", ans);
}
