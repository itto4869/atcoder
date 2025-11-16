use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        a: usize,
        _: usize,
        s: Bytes,
    }
    for (i, &c) in s.iter().enumerate() {
        if i == a && c == b'-' {
            continue;
        } else if i == a && c != b'-' {
            println!("No");
            return;
        } else if c == b'-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
