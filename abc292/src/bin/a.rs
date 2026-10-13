use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    println!("{}", s.to_ascii_uppercase());
}
