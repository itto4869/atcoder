use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64
    }
    if a <= c {
        if b <= d {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        println!("No");
    }
}
