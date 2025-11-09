use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut cnt = 0;
    for &si in &s {
        if si == b'1' {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
