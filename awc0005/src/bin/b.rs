use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        mut s: [u64; n],
    }
    for _ in 0..m {
        input! {
            p: Usize1,
            v: u64,
        }
        s[p] = v;
    }

    let ans = s.iter().filter(|&&x| x < k).count();
    println!("{}", ans);
}
