use cp_library::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [Usize1; n],
    }
    let mut ok = true;
    for i in 0..n {
        if i != b[a[i]] {
            ok = false;
        }
    }

    yes_no!(ok);
}
