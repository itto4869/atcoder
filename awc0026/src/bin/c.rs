use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: u64,
        e: u64,
        mut p: [u64; n],
    }
    p.sort_unstable();

    let mut ans = 0;
    let mut curr = 0;
    for &pi in &p {
        curr += pi * t;
        if curr <= e {
            ans += 1;
        }
    }

    println!("{}", ans);
}
