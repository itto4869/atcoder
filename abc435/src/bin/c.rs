use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut idx = 0;
    let mut target = a[idx];
    while idx < target && idx < n {
        target = target.max(idx + a[idx]);
        idx += 1;
    }

    println!("{}", target.min(n));
}
