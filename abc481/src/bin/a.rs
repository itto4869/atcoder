use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let d = (a + b).ilog10() + 1;
    println!("{}", d);
}
