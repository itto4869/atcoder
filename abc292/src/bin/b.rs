use cp_utils::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v = vec![0; n];
    for _ in 0..q {
        input! {
            c: usize,
            x: Usize1,
        }
        if c == 1 {
            v[x] += 1;
        } else if c == 2 {
            v[x] += 2;
        } else {
            yes_no!(v[x] >= 2);
        }
    }
}
