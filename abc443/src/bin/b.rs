use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize,
        k: usize,
    }
    let mut sum = n;
    n += 1;
    let mut i = 0;
    while sum < k {
        sum += n;
        i += 1;
        n += 1;
    }

    println!("{}", i);
}
