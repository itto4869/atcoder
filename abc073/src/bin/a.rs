use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    if n.contains(&'9') {
        println!("Yes");
    } else {
        println!("No");
    }
}
