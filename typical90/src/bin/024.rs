use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut cnt = 0;
    for i in 0..n {
        cnt += a[i].abs_diff(b[i]);
    }

    if k < cnt {
        println!("No");
    } else {
        yes_no!((k - cnt) % 2 == 0);
    }
}
