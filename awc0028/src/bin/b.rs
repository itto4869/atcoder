use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: u64,
        r: u64,
    }
    let mut ans = 0;
    let mut cnt = 0;
    for _ in 0..n {
        input! {
            t: u64,
        }
        if l <= t && t <= r {
            cnt += 1;
            ans = ans.max(cnt);
        } else {
            cnt = 0;
        }
    }

    ans = ans.max(cnt);
    println!("{}", ans);
}
