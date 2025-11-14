use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let l = a + b;
    let r = c + d;
    if l > r {
        println!("Left");
    } else if l < r {
        println!("Right");
    } else {
        println!("Balanced");
    }
}
