use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();
    a.reverse();
    
    let ans = a[0] + a[1];
    println!("{}", ans);
}
