use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64
    }
    if a == c {
        println!("{}", b);
    } else if a == b {
        println!("{}", c);
    } else {
        println!("{}", a);
    }
}
