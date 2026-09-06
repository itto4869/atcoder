use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }
    for i in 1..=3 {
        if x != i {
            println!("{}", i);
            return;
        }
    }
}
