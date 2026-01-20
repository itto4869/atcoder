use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
        m: usize,
    }
    let mut x = 0u128;
    for i in 0..=m {
        x += n.pow(i as u32);
        if x > 10u128.pow(9) {
            println!("inf");
            return;
        }
    }

    println!("{}", x);
}
