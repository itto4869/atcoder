use proconio::{fastout, input, marker::{Bytes, Usize1}};

#[fastout]
fn main() {
    input! {
        _: usize,
        l: Usize1,
        r: usize,
        s: Bytes,
    }
    for i in l..r {
        if s[i] == b'o' {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
