use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: Bytes,
    }
    if n[0] == n[1] && n[1] == n[2] {
        println!("Yes");
    } else if n[1] == n[2] && n[2] == n[3] {
        println!("Yes");
    } else {
        println!("No");
    }
}
