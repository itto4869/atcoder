use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: Bytes,
        a: Bytes,
    }

    for i in 0..n {
        if t[i] == a[i] && t[i] == b'o' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
