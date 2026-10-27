use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [usize; 5 * n],
    }
    x.sort_unstable();
    let ans = x[n..(4 * n)].iter().sum::<usize>() as f64 / (3 * n) as f64;
    println!("{:.15}", ans);
}
