
use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    if a == b {
        yes_no!(true);
        return;
    }

    let l = (0..n).find(|&i| a[i] != b[i]).unwrap();
    let r = (0..n).rfind(|&i| a[i] != b[i]).unwrap();

    let right = a[l] == b[r]
        && (l..r).all(|i| a[i + 1] == b[i]);

    let left = a[r] == b[l]
        && (l + 1..=r).all(|i| a[i - 1] == b[i]);

    yes_no!(right || left);
}
