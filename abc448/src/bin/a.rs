use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: u64,
        a: [u64; n],
    }
    for &ai in &a {
        if ai < x {
            x = ai;
            println!("1");
        } else {
            println!("0");
        }
    }
}
