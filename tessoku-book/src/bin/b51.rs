use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut l_stack = Vec::new();
    for i in 0..s.len() {
        if s[i] == b'(' {
            l_stack.push(i + 1);
        } else {
            println!("{} {}", l_stack.pop().unwrap(), i + 1);
        }
    }
}
