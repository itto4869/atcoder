use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
        x: u64,
    }
    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
