use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    for i in (0..s.len()).step_by(2) {
        print!("{}", s[i] as char);
    }
}
