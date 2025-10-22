use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut num = 0;
    for &(l, r) in &lr {
        num += r - l + 1;
    }
    println!("{}", num);
}
