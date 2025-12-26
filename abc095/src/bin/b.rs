use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
        mut m: [u64; n],
    }
    let m_sum: u64 = m.iter().sum();
    let m_rest = x - m_sum;
    m.sort();
    let ans = n + (m_rest / m[0]) as usize;
    println!("{}", ans);
}
