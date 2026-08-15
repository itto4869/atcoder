use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let sum = a + b;
    for i in 1..=n {
        input! {
            c: usize,
        }
        if sum == c {
            println!("{}", i);
            break;
        }
    }
}
