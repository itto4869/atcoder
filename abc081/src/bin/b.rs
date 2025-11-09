use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = u64::MAX;
    for &ai in &a {
        let mut b = ai;
        let mut cnt = 0;
        while (b % 2) == 0 {
            cnt += 1;
            b /= 2;
        } 
        ans = ans.min(cnt);
    }
    println!("{}", ans);
}
