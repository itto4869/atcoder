use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
        z: u64,
    }
    for i in 0..100000 {
        if (x + i) % (y + i) == 0 && (x + i) / (y + i) == z {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
