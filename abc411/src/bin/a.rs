use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        p: Bytes,
        l: usize
    }
    if p.len() >= l {
        println!("Yes");
    } else {
        println!("No");
    }
}
