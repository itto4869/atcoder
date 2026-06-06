use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p: char,
        q: char,
    }
    let mut d = [0, 3, 1, 4, 1, 5, 9];
    for i in 1..d.len() {
        d[i] += d[i - 1];
    }

    let (pi, qi) = ((p as u32 - 'A' as u32) as usize, (q as u32 - 'A' as u32) as usize);
    let ans = d[pi.max(qi)] - d[pi.min(qi)];
    println!("{}", ans);
}
