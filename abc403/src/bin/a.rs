use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let ans: u64 = a.iter().step_by(2).sum();
    println!("{}", ans);
}
