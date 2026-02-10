use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }
    let idx = a.iter().position(|&x| x == k);
    if let Some(ans) = idx {
        println!("{}", ans + 1);
    } else {
        println!("-1");
    }
}
