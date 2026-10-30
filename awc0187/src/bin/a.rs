use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        let res = (a + b - 1) / b;
        println!("{}", res);
    }
}
