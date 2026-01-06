use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; n - 1],
        a: [u64; n - 1]
    }
    let mut dp: Vec<u64> = vec![1 << 60; n];
    dp[0] = 0;
    for i in 1..n {
        for j in i.saturating_sub(c[i - 1])..i {
            dp[i] = dp[i].min(dp[j] + 1);
        }
    }

    let mut ans = 0;
    let mut has_been = vec![false; n];
    for i in 1..n {
        if a[i - 1] > 0 {
            has_been[i] = true;
        }
    }
    for i in (1..n).rev() {
        if has_been[i] {
            ans += 1;
            let mut ok = false;
            let mut min_idx = 0;
            let mut min_val = 1 << 60;
            for j in i.saturating_sub(c[i - 1])..i {
                if min_val >= dp[j] {
                    min_val = dp[j];
                    min_idx = j;
                }
                if has_been[j] {
                    ok = true;
                    break;
                }
            }

            if !ok {
                has_been[min_idx] = true;
            }
        }
    }

    println!("{}", ans);
}