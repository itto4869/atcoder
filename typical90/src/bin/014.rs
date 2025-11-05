use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    
    let mut ans = 0;
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        ans += ai.abs_diff(bi);
    }
    println!("{}", ans);
}
