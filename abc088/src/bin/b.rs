use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort();
    a.reverse();
    let point_a: u64 = a.iter().step_by(2).sum();
    let point_b: u64 = a.iter().skip(1).step_by(2).sum();
    println!("{}", point_a - point_b);
}
