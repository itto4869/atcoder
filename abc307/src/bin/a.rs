use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    }
    println!("{}", a.chunks(7).map(|chunk| chunk.iter().sum::<usize>()).format(" "));
}
