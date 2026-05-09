use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: Usize1,
    }
    println!("{}", a[x]);
}
