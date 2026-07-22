use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let d = n.ilog10();
    let k = 10usize.pow(d.saturating_sub(2));
    let ans = (n / k) * k;
    println!("{}", ans);
}
